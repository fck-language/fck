//! Contains the lexer and parser for fck. There are language agnostic and as such cannot use regex.

use crate::bases::*;
use crate::err_wrn::*;
use lang::keywords::Keywords;
use crate::nodes::{ASTNode, ASTNodeType};
use lang::get_associated_keywords;
use std::collections::HashMap;
use type_things::symbol_tables::SymbolTable;

/// Lexer for fck
///
/// This works on a (8-bit) character by character basis. 16-bit characters are still in production
/// so be patient
pub struct Lexer {
    /// Input text split into a vector of characters
    split_text: Vec<char>,
    /// Current position of the "pointer"
    current_pos: Position,
    /// The current index of the current character in `split_text`
    char_index: usize,
    /// the current character the pointer is pointing to
    current_char: char,
    /// The current keyword struct words are being checked against
    pub(crate) keywords: Keywords<'static>,
    /// The string associated with the current keywords
    pub(crate) keyword_code: String,
    comment_handler: fn(&mut Lexer) -> Option<Token>
}

impl Lexer {
    /// Returns a new lexer with all fields completed appropriately
    pub fn new(full_text: String, keywords: Keywords<'static>, keyword_code: String, include_comments: bool) -> Lexer {
        return Lexer {
            split_text: full_text.chars().collect(),
            current_pos: Position::new(),
            char_index: 0,
            current_char: full_text.chars().nth(0).unwrap_or(char::from(0)),
            keywords,
            keyword_code,
            comment_handler: if include_comments { Lexer::include_comment } else { Lexer::skip_comment }
        };
    }

    /// Advances the `self.current_pos` by 1 and updates the character it points to
    fn advance(&mut self) {
        self.current_pos = self.current_pos.advance();
        self.char_index += 1;
        self.current_char = *self.split_text.get(self.char_index).unwrap_or(&char::from(0));
    }

    /// Primary function of the lexer. Iterates over the input text and attempts to turn it into a
    /// vector of tokens. If an error occurs, an `Err(Error)` is returned
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![];
        while self.char_index < self.split_text.len() {
            if " \t".contains(self.current_char) {
                self.advance();
            } else if self.current_char.is_numeric() || self.current_char == '.' {
                tokens.push(self.make_number());
            } else if self.current_char.is_alphabetic() {
                tokens.push(self.make_identifier());
            } else {
                let pos_start = self.current_pos.clone();
                if self.current_char == '\n' {
                    if !(self.char_index == self.split_text.len() - 1) {
                        tokens.push(Token::new(TokType::Newline, pos_start,
                                               self.current_pos.clone()));
                        self.current_pos.advance_ln();
                    }
                    self.advance();
                } else if self.current_char == '#' {
                    self.advance();
                    if let Some(tok) = (self.comment_handler)(self) { tokens.push(tok) };
                } else if self.current_char == '!' {
                    self.advance();
                    if self.current_char == '!' {
                        self.advance();
                        let lang_code = match self.make_identifier().type_ {
                            TokType::Identifier(_, v) => v,
                            _ => unreachable!()
                        };
                        match get_associated_keywords(lang_code.as_str()) {
                            Some(k) => self.keywords = k,
                            None => return Err(Error::new(pos_start,
                                                          self.current_pos.clone(),
                                                          0101))
                        }
                        self.keyword_code = lang_code;
                        if self.current_char == '\n' {
                            self.current_pos.advance_ln();
                        }
                        self.advance();
                    } else {
                        match self.make_not_equals(pos_start) {
                            Ok(t) => tokens.push(t),
                            Err(e) => return Err(e)
                        }
                    }
                } else {
                    let tok_type_opt = match self.current_char {
                        '(' => Some(TokType::LParen),
                        ')' => Some(TokType::RParen),
                        '{' => Some(TokType::LParenCurly),
                        '}' => Some(TokType::RParenCurly),
                        ',' => Some(TokType::Comma),
                        '[' => Some(TokType::LParenSquare),
                        ']' => Some(TokType::RParenSquare),
                        ';' => Some(TokType::Newline),
                        '?' => Some(TokType::QuestionMark),
                        '.' => Some(TokType::Dot),
                        ':' => Some(TokType::Colon),
                        _ => None
                    };
                    if let Some(tok_type) = tok_type_opt {
                        self.advance();
                        tokens.push(Token::new(tok_type, pos_start,
                                               self.current_pos.clone()));
                        continue;
                    }

                    let tok = match match self.current_char {
                        '+' | '-' | '*' | '/' | '%' => self.make_op_or_set(),
                        '<' => self.single_double_token('=', TokType::LT, TokType::LTE),
                        '>' => self.single_double_token('=', TokType::GT, TokType::GTE),
                        '=' => self.make_equals(),
                        '@' => self.make_loop_identifier(),
                        '\'' | '"' => self.make_string(self.current_char),
                        _ => Err(Error::new(pos_start, pos_start.clone().advance(), 0201))
                    } {
                        Ok(tok) => tok,
                        Err(e) => return Err(e)
                    };
                    tokens.push(tok);
                }
            }
        }
        Ok(tokens)
    }

    /// Called by `self.make_tokens` when a number is found. Iterates over characters until a
    /// second '.' or non-numeric character is found, at which point the function returns the
    /// appropriate token
    fn make_number(&mut self) -> Token {
        let mut has_dot = false;
        let pos_start = self.current_pos.clone();
        let mut value = String::new();

        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                if has_dot {
                    break;
                }
                has_dot = true;
                value.push('.')
            } else {
                value.push(self.current_char)
            }
            self.advance()
        }

        return Token::new(
            if has_dot {
                TokType::Float(value.parse::<f64>().unwrap())
            } else {
                TokType::Int(value.parse::<u64>().unwrap())
            },
            pos_start,
            self.current_pos.clone()
        );
    }
    
    /// Called by `self.make_tokens` when a alphabetic character is found. This function either
    /// returns a `TT_KEYWORD` or `TT_IDENTIFIER` token depending on the string and current keywords
    fn make_identifier(&mut self) -> Token {
        let pos_start = self.current_pos.clone();
        let mut keyword = self.current_char.to_string();
        self.advance();
        
        while (self.current_char.is_alphanumeric() || self.current_char == '_') &&
            self.char_index < self.split_text.len() {
            keyword.push(self.current_char);
            self.advance();
        }
        
        let tok_type = match self.keywords.contains(&keyword) {
            Some((list, pos)) => TokType::Keyword(list, pos),
            None => TokType::Identifier(self.keyword_code.clone(), keyword)
        };
        
        return Token::new(tok_type, pos_start, self.current_pos.clone());
    }
    
    /// Called by `self.make_tokens` when a '@' character is found. This advances and calls
    /// `self.make_identifier`, taking the value of the token returned and placing it into a new
    /// `TT_AT` token
    fn make_loop_identifier(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        let id = self.make_identifier();
        match id.type_ {
            TokType::Identifier(_, v) => Ok(
                Token::new(TokType::Label(v),
                           pos_start,
                           self.current_pos.clone())
            ),
            TokType::Keyword(_, _) => Err(
                Error::new(id.pos_start, id.pos_end, 0401)
            ),
            _ => unreachable!()
        }
    }

    /// Called by `self.make_tokens` when an opening string delimiter is found. Parses the string
    /// and returns a `TT_STRING` token containing the string literal in an `Ok(Token)`. Returns an
    /// `Err(Error)` if no closing delimiter is found.
    fn make_string(&mut self, starting_character: char) -> Result<Token, Error> {
        let mut out = String::new();
        let pos_start = self.current_pos.clone().clone();
        let mut escape = false;
        let escape_chars: HashMap<char, char> = [('n', '\n'), ('t', '\t'), ('r', '\r')]
            .iter().cloned().collect();

        self.advance();
        while self.current_char != starting_character {
            if escape {
                out += &*format!("{}", match escape_chars.get(&self.current_char) {
                    Some(c) => c.clone(),
                    None => self.current_char
                });
                escape = false;
            } else {
                if self.current_char == '\\' {
                    escape = true;
                } else {
                    out += &*format!("{}", self.current_char)
                }
            }
            self.advance()
        }
        self.advance();
        Ok(Token::new(TokType::String(out), pos_start, self.current_pos.clone()))
    }

    /// Called by `self.make_tokens` when a '!' is found. Will check for a "!=", "!!", or character
    /// after it otherwise an `Err(Error)` is returned.
    fn make_not_equals(&mut self, pos_start: Position) -> Result<Token, Error> {
        if self.current_char == '=' {
            self.advance();
            return Ok(Token::new(TokType::NE, pos_start,
                                 self.current_pos.clone()));
        } else if self.current_char.is_alphabetic() || "_!".contains(self.current_char) {
            return Ok(Token::new(TokType::Not, pos_start,
                                 self.current_pos.clone()));
        }
        return Err(Error::new(pos_start, self.current_pos.clone(), 0202));
    }

    /// Called by `self.make_tokens` when a '=' is found. Will check for a "==" otherwise an
    /// `Err(Error)` is returned
    fn make_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Ok(
                Token::new(TokType::Eq,
                           pos_start,
                           self.current_pos.clone()));
        }
        return Ok(Token::new(TokType::Set, pos_start, self.current_pos.clone()));
    }

    /// General function called by `self.make_tokens` when we want to check for one of two tokens
    /// that start with the same character with the second token being followed by another character
    /// such as `//` or `<=`
    fn single_double_token(&mut self, second_char: char, single_type: TokType, double_type: TokType) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        let tok_type;
        if self.current_char == second_char {
            self.advance();
            tok_type = double_type;
        } else {
            tok_type = single_type;
        }
        return Ok(Token::new(tok_type, pos_start,
                             self.current_pos.clone()));
    }

    /// Will return either an operation token, or reassignment token (such as `+=`). This is wrapped
    /// in a `Result`
    fn make_op_or_set(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        match self.current_char {
            '+' => {
                self.advance();
                if self.current_char == '+' {
                    self.advance();
                    return Ok(Token::new(TokType::Increment, pos_start, self.current_pos.clone()))
                } else if self.current_char == '=' {
                    self.advance();
                    return Ok(Token::new(TokType::SetPlus, pos_start, self.current_pos.clone()))
                } else {
                    return Ok(Token::new(TokType::Plus, pos_start, self.current_pos.clone()))
                }
            }
            '-' => {
                self.advance();
                if self.current_char == '-' {
                    self.advance();
                    return Ok(Token::new(TokType::Decrement, pos_start, self.current_pos.clone()))
                } else if self.current_char == '=' {
                    self.advance();
                    return Ok(Token::new(TokType::SetMinus, pos_start, self.current_pos.clone()))
                } else {
                    return Ok(Token::new(TokType::Minus, pos_start, self.current_pos.clone()))
                }
            }
            '%' => {
                self.advance();
                if self.current_char == '=' {
                    self.advance();
                    return Ok(Token::new(TokType::SetMod, pos_start, self.current_pos.clone()))
                } else {
                    return Ok(Token::new(TokType::Mod, pos_start, self.current_pos.clone()))
                }
            }
            '*' => {
                self.advance();
                if self.current_char == '*' {
                    self.advance();
                    if self.current_char == '=' {
                        self.advance();
                        return Ok(Token::new(TokType::SetPow, pos_start, self.current_pos.clone()))
                    } else {
                        return Ok(Token::new(TokType::Pow, pos_start, self.current_pos.clone()))
                    }
                } else if self.current_char == '=' {
                    self.advance();
                    return Ok(Token::new(TokType::SetMult, pos_start, self.current_pos.clone()))
                } else {
                    return Ok(Token::new(TokType::Mult, pos_start, self.current_pos.clone()))
                }
            }
            '/' => {
                self.advance();
                if self.current_char == '/' {
                    self.advance();
                    if self.current_char == '=' {
                        self.advance();
                        return Ok(Token::new(TokType::SetFDiv, pos_start, self.current_pos.clone()))
                    } else {
                        return Ok(Token::new(TokType::FDiv, pos_start, self.current_pos.clone()))
                    }
                } else if self.current_char == '=' {
                    self.advance();
                    return Ok(Token::new(TokType::SetDiv, pos_start, self.current_pos.clone()))
                } else {
                    return Ok(Token::new(TokType::Div, pos_start, self.current_pos.clone()))
                }
            }
            _ => unreachable!()
        }
    }

    /// Called by `self.make_tokens` when a comment is found if comments are being ignored. This
    /// will skip over any comment
    fn skip_comment(&mut self) -> Option<Token> {
        if self.current_char == '#' {
            self.advance();
            while !(self.current_char == '\n' || self.char_index + 1 == self.split_text.len()) {
                self.advance();
            }
            self.current_pos.advance_ln();
        } else {
            while self.current_char != '#' && self.char_index + 1 != self.split_text.len() {
                self.advance();
            }
        }
        self.advance();
        None
    }
    
    /// Called by `self.make_tokens` when a comment is found if comments are being included. This
    /// will include the comment (with the current language key) for the translator to use
    fn include_comment(&mut self) -> Option<Token> {
        let mut pos_start = self.current_pos.clone();
        pos_start.col -= 1;
        let mut out = String::new();
        if self.current_char == '#' {
            self.advance();
            while !(self.current_char == '\n' || self.char_index + 1 == self.split_text.len()) {
                out.push(self.current_char);
                self.advance();
            }
            self.current_pos.advance_ln();
        } else {
            while self.current_char != '#' && self.char_index + 1 != self.split_text.len() {
                out.push(self.current_char);
                self.advance();
            }
        }
        self.advance();
        Some(Token::new(TokType::Comment(out, self.keyword_code.clone()), pos_start, self.current_pos.clone()))
    }
}

/// Parser for fck
///
/// Parses a `Vec<Token>` from the lexer and produces an AST (technically it produces an annotated
/// AST but there we go)
pub struct Parser {
    /// Vector of tokens current un-processed
    to_process: Vec<Token>,
    /// An intermediate storage for tokens essentially functioning as a back-up
    intermediate: Vec<Token>,
    /// Current token being processed
    current_tok: Option<Token>,
    /// Used to store the previous end of the last processed token
    previous_end: Position,
    /// Represents if the current token can be discarded after processing or if it needs to be added
    /// to `self.intermediate` in case the parser needs to back-track
    safe: bool,
    /// All the symbol tables for all the scopes. We got them all. Symbol tables, other symbol
    /// tables, more shit, other things. Anything you could want
    symbol_tables: Vec<SymbolTable>,
}

impl Parser {
    /// Makes a new parser from a `Vec<Token>`. Reverses the token vector so that the popped token
    /// from the vector is the first token in the original vector
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens;
        tokens.reverse();
        let current_tok = tokens.clone().pop().unwrap();
        return Parser {
            to_process: tokens,
            intermediate: vec![],
            current_tok: Some(current_tok.clone()),
            previous_end: current_tok.pos_start,
            safe: false,
            symbol_tables: vec![SymbolTable::new()],
        };
    }

    // Navigating tokens functions
    /// Advances the current token to the next token. Puts the previous token into
    /// `self.intermediate` if `self.safe` is `true`. Also updates `self.previous_end`
    fn next(&mut self) {
        if self.current_tok.is_none() {
            return;
        } else if self.safe {
            self.intermediate.push(self.current_tok.clone().unwrap());
        }
        self.previous_end = self.current_tok.clone().unwrap().pos_end;
        self.current_tok = self.to_process.pop();
    }

    /// Called when the `self.safe` can be changed to `false`. Clears the intermediate and sets
    /// `self.safe` to `false`
    fn finalise(&mut self) {
        self.intermediate.clear();
        self.safe = false;
    }

    /// Called when the parser needs to back-track. Places the current token into the intermediate
    /// then places the intermediate into `self.to_process` and then clears the intermediate. Also
    /// resets `self.safe` to `false`
    fn put_back(&mut self) {
        self.intermediate.push(self.current_tok.clone().unwrap());

        self.intermediate.reverse();
        self.to_process.append(&mut self.intermediate);
        self.intermediate.clear();
        self.safe = false;
        self.next();
    }

    // Symbol table things
    fn new_scope(&mut self, name: Option<String>) {
        self.symbol_tables.push(
            self.symbol_tables.last().unwrap().new_child(
                self.symbol_tables.len(), name
            )
        );
    }

    // General useful functions
    /// Skips new lines. Don't care about them
    fn skip_newlines(&mut self) {
        while self.current_tok.is_some() && self.current_tok.clone().unwrap() == TokType::Newline {
            self.next()
        }
    }

    // Parsing tokens
    /// Main function. Parses the tokens and returns an `Ok(Vec<ASTNode>)` if no errors were found.
    /// Otherwise returns a `Err(Error)`.
    ///
    /// The reason it returns a `Vec<ASTNode>` not just an `ASTNode` is because one piece of code
    /// cannot be placed into a single AST. The ASTs are ordered. For example
    /// ```fck
    /// int a = 5
    /// float b = 12
    /// ```
    /// is two ASTs, both performing variable assignments
    pub fn parse(&mut self) -> Result<(Vec<ASTNode>, Vec<SymbolTable>), Error> {
        let mut out_ast: Vec<ASTNode> = vec![];

        // Loop through tokens to create an ast
        loop {
            self.next();
            if self.current_tok.is_none() { break; }

            self.skip_newlines();
            if self.current_tok.is_none() { break; }

            match self.statement() {
                Ok(ast) => out_ast.push(ast),
                Err(error) => return Err(error)
            }
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TokType::Newline {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start,
                                      tok.pos_end,
                                      0301,
                ));
            }
        }
        let mut st_out = self.symbol_tables.clone();
        st_out.reverse();
        return Ok((out_ast, st_out));
    }

    /// First grammar rule
    /// ```regex
    /// [return|continue|break|(expr)]
    /// ```
    fn statement(&mut self) -> Result<ASTNode, Error> {
        let pos_start = self.current_tok.clone().unwrap().pos_start.clone();
        let mut pos_end = self.current_tok.clone().unwrap().pos_end.clone();
        let tok = self.current_tok.clone().unwrap();

        // return keyword
        if tok == TokType::Keyword(0, 15) {
            self.next();
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                return Ok(ASTNode::new_v(ASTNodeType::Return(false),
                                       pos_start,
                                       pos_end));
            } else if self.current_tok.clone().unwrap() == TokType::Keyword(0, 3) /* if */{
                self.safe = true;
                self.next();
                println!("{:?}", self.intermediate);
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0302));
                }
                let conditional = match self.comp_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                };
                return if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    self.safe = false;
                    self.finalise();
                    Ok(ASTNode::new(ASTNodeType::Return(true),
                                    vec![conditional],
                                    pos_start,
                                    pos_end))
                } else if self.current_tok.clone().unwrap().type_ != TokType::LParenCurly {
                    Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, 0303))
                } else {
                    println!("{:?}", self.intermediate);
                    self.put_back();
                    println!("{:?}", self.current_tok.clone().unwrap());
                    let ret_value = match self.expr() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    };
                    pos_end = ret_value.clone().pos_end;
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                        Ok(ASTNode::new(ASTNodeType::Return(false),
                                        vec![ret_value],
                                        pos_start,
                                        pos_end))
                    } else if self.current_tok.clone().unwrap() == TokType::Keyword(0, 3) {
                        self.next();
                        let conditional = match self.comp_expr() {
                            Ok(n) => n,
                            Err(e) => return Err(e)
                        };
                        pos_end = conditional.clone().pos_end;
                        Ok(ASTNode::new(ASTNodeType::Return(true),
                                        vec![ret_value, conditional],
                                        pos_start,
                                        pos_end))
                    } else {
                        Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, 0301))
                    }
                };
            }
            let expr = match self.expr() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };

            return if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                pos_end = expr.pos_end.clone();
                Ok(ASTNode::new(ASTNodeType::Return(false),
                                Vec::from([expr]),
                                pos_start,
                                pos_end))
            } else if self.current_tok.clone().unwrap() == TokType::Keyword(0, 3) {
                self.next();
                let conditional = match self.comp_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                };
                pos_end = conditional.clone().pos_end;
                Ok(ASTNode::new(ASTNodeType::Return(true),
                                vec![expr, conditional],
                                pos_start,
                                pos_end))
            } else {
                Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, 0301))
            };
        }

        // continue(0.16) and break(0.17) keywords
        // TODO: The fuck is this?
        let mut out;
        if let TokType::Label(v) = tok.type_ {
            self.safe = true;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0310))
            }
            let op = self.current_tok.clone().unwrap();
            if let TokType::Keyword(0, index) = op.type_ {
                if index == 16 || index == 17 {
                    self.finalise();
                    out = ASTNode::new_v(match index {
                        16 => ASTNodeType::Continue(Some(v)),
                        17 => ASTNodeType::Break(Some(v)),
                        _ => unreachable!()
                    }, tok.pos_start, op.pos_end);
                    self.next();
                } else {
                    self.put_back();
                    return self.expr()
                }
            } else {
                self.put_back();
                return self.expr()
            }
        } else if let TokType::Keyword(0, index) = tok.type_ {
            if index == 16 || index == 17 {
                out = ASTNode::new_v(match index {
                    16 => ASTNodeType::Continue(None),
                    17 => ASTNodeType::Break(None),
                    _ => unreachable!()
                }, tok.pos_start, tok.pos_end);
                self.next();
            } else {
                return self.expr()
            }
        } else {
            return self.expr()
        }
        if self.current_tok.is_some() {
            if self.current_tok.clone().unwrap() != TokType::Keyword(0, 3) {
                return Err(Error::new(pos_start, self.current_tok.clone().unwrap().pos_end, 0302));
            }
            self.next();
            out.child_nodes = vec![match self.comp_expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            }];
            if self.current_tok.is_some() {
                return Err(Error::new(pos_start, self.current_tok.clone().unwrap().pos_end, 0301));
            }
        }
        if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
            Ok(out)
        } else {
            Err(Error::new(pos_start, pos_end, 0301))
        }
    }

    fn expr(&mut self) -> Result<ASTNode, Error> {
        let mut tok = self.current_tok.clone().unwrap();
        let default_values = |x: u16| match x {
            1 => ASTNodeType::Int(0),
            2 => ASTNodeType::Float(0.),
            3 => ASTNodeType::Bool(false),
            4 => ASTNodeType::String(String::new()),
            5 => ASTNodeType::List,
            _ => unreachable!()
        };

        let pos_start = tok.pos_start.clone();
        let mut pos_end = tok.pos_end.clone();

        // check for new variable assignments
        if tok == 1 {
            let var_type = match tok.type_ {
                TokType::Keyword(_, v) => v as u16 + 1, // Add 1 because type 0 is null
                _ => unreachable!()
            };
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance().clone(), 0304));
            };
            tok = self.current_tok.clone().unwrap();
            let var_name;
            if let TokType::Identifier(lang, v) = tok.type_ {
                var_name = format!("{}:{}", lang, v);
            } else {
                return Err(Error::new(tok.pos_start, tok.pos_end, 0304));
            }
            pos_end = tok.pos_end.clone();
            let expr: ASTNode;

            self.next();
            if self.current_tok.is_none() {
                expr = ASTNode::new_v(default_values(var_type),
                                    Position::new(),
                                    Position::new());
                return Ok(ASTNode::new(ASTNodeType::VarAssign(var_type, var_name),
                                       Vec::from([expr]),
                                       pos_start,
                                       pos_end));
            };

            let mut pos_end = self.current_tok.clone().unwrap().pos_end;
            match self.current_tok.clone().unwrap().type_ {
                TokType::Set => {
                    self.next();
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                        expr = ASTNode::new_v(default_values(var_type),
                                            Position::new(),
                                            Position::new());
                        if self.current_tok.clone().unwrap().type_ == TokType::Newline {
                            self.next();
                        }
                    } else {
                        expr = match self.expr() {
                            Ok(ast) => ast,
                            Err(e) => return Err(e)
                        };
                        pos_end = expr.pos_end;
                    }
                }
                TokType::Newline => {
                    expr = ASTNode::new_v(default_values(var_type),
                                        Position::new(),
                                        Position::new());
                }
                _ => return Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_start, 0306))
            }
            let mut last = self.symbol_tables.pop().unwrap();
            last.push(var_name.clone());
            self.symbol_tables.push(last);
            return Ok(ASTNode::new(ASTNodeType::VarAssign(var_type, var_name),
                                   Vec::from([expr]),
                                   pos_start,
                                   pos_end));
        };

        // TODO: static(previously silent) variable assignments

        let node = match self.comp_expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        // 0.0 is 'and', 0.1 is 'or'
        if self.current_tok.clone().unwrap() == TokType::Keyword(0, 0) || self.current_tok.clone().unwrap() == TokType::Keyword(0, 1) {
            let mut children = vec![node];
            let mut operators = String::new();
            while self.current_tok.clone().unwrap() == TokType::Keyword(0, 0) || self.current_tok.clone().unwrap() == TokType::Keyword(0, 1) {
                operators += if self.current_tok.clone().unwrap() == TokType::Keyword(0, 0) { "&" } else { "|" };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                }
                children.push(match self.comp_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.last().unwrap().clone().pos_end;
            return Ok(ASTNode::new(ASTNodeType::CompOp(operators.chars().collect::<Vec<char>>()),
                                   children,
                                   pos_start,
                                   pos_end));
        }

        if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TokType::QuestionMark {
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
            }
            let mut children = vec![node.clone()];
            return if self.current_tok.clone().unwrap().type_ == TokType::Colon {
                // Only the false option given
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                } else if self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    let tok = self.current_tok.clone().unwrap();
                    return Err(Error::new(tok.pos_start, tok.pos_end, 0305));
                }

                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });

                pos_end = children.clone().pop().unwrap().pos_end;
                Ok(ASTNode::new(ASTNodeType::Ternary(false, true), children, node.pos_start, pos_end))
            } else {
                // true option definitely given
                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                pos_end = children.clone().pop().unwrap().pos_end;
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0307));
                } else if self.current_tok.clone().unwrap().type_ != TokType::Colon {
                    let tok = self.current_tok.clone().unwrap();
                    return Err(Error::new(tok.pos_start, tok.pos_end, 0307));
                }
                self.next();
                if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    // Only the true option given
                    return Ok(ASTNode::new(ASTNodeType::Ternary(true, false), children, pos_start, pos_end));
                }
                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                pos_end = children.clone().pop().unwrap().pos_end;
                Ok(ASTNode::new(ASTNodeType::Ternary(true, true), children, pos_start, pos_end))
            };
        }

        Ok(node)
    }

    fn comp_expr(&mut self) -> Result<ASTNode, Error> {
        // 0.2 is 'not'
        if self.current_tok.clone().unwrap() == TokType::Keyword(0, 2) {
            let pos_start = self.current_tok.clone().unwrap().pos_start;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0305));
            }
            let node = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(ASTNode::new(ASTNodeType::UnaryNot,
                                   vec![node.clone()],
                                   pos_start,
                                   node.pos_end,
            ));
        }

        let node = match self.arith_expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        if self.current_tok.clone().unwrap().type_ == TokType::Eq ||
                self.current_tok.clone().unwrap().type_ == TokType::NE ||
                self.current_tok.clone().unwrap().type_ == TokType::LT ||
                self.current_tok.clone().unwrap().type_ == TokType::GT ||
                self.current_tok.clone().unwrap().type_ == TokType::LTE ||
                self.current_tok.clone().unwrap().type_ == TokType::GTE {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TokType::Eq ||
                self.current_tok.clone().unwrap().type_ == TokType::NE ||
                self.current_tok.clone().unwrap().type_ == TokType::LT ||
                self.current_tok.clone().unwrap().type_ == TokType::GT ||
                self.current_tok.clone().unwrap().type_ == TokType::LTE ||
                self.current_tok.clone().unwrap().type_ == TokType::GTE {
                operators += match self.current_tok.clone().unwrap().type_ {
                    TokType::Eq => "e", // ==
                    TokType::NE => "n", // !=
                    TokType::LT => "l", // <
                    TokType::GT => "g", // >
                    TokType::LTE => "L", // <=
                    TokType::GTE => "G", // >=
                    _ => unreachable!()
                };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                }
                children.push(match self.arith_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.last().unwrap().clone().pos_end;
            return Ok(ASTNode::new(ASTNodeType::CompOp(operators.chars().collect::<Vec<char>>()),
                                   children,
                                   pos_start,
                                   pos_end));
        } else if self.current_tok.clone().unwrap() == TokType::Keyword(0, 10) {
            let start_node = node;
            let pos_start = start_node.clone().pos_start;
            self.next();
            // End value of the range statement
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
            }
            let end_node = match self.arith_expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            // Checking for the end of the range statement
            if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::Keyword(0, 12) {
                return Ok(ASTNode::new(ASTNodeType::Range, vec![start_node, end_node.clone()], pos_start, end_node.pos_end));
            }
            self.next();
            // Getting the step value
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
            }
            let step = match self.arith_expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(ASTNode::new(ASTNodeType::Range, vec![start_node, end_node, step.clone()], pos_start, step.pos_end));
        }

        Ok(node)
    }

    fn arith_expr(&mut self) -> Result<ASTNode, Error> {
        let node = match self.term() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        if self.current_tok.clone().unwrap().type_ == TokType::Plus || self.current_tok.clone().unwrap().type_ == TokType::Minus {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TokType::Plus || self.current_tok.clone().unwrap().type_ == TokType::Minus {
                operators += if self.current_tok.clone().unwrap().type_ == TokType::Plus { "+" } else { "-" };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                }
                children.push(match self.term() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.clone().last().unwrap().pos_end;
            return Ok(ASTNode::new(ASTNodeType::ArithOp(operators.chars().collect::<Vec<char>>()),
                                   children,
                                   pos_start,
                                   pos_end));
        }

        Ok(node)
    }

    fn term(&mut self) -> Result<ASTNode, Error> {
        let node = match self.power() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        if self.current_tok.clone().unwrap().type_ == TokType::Mult ||
            self.current_tok.clone().unwrap().type_ == TokType::Div ||
            self.current_tok.clone().unwrap().type_ == TokType::FDiv ||
            self.current_tok.clone().unwrap().type_ == TokType::Mod {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TokType::Mult ||
                self.current_tok.clone().unwrap().type_ == TokType::Div ||
                self.current_tok.clone().unwrap().type_ == TokType::FDiv ||
                self.current_tok.clone().unwrap().type_ == TokType::Mod {
                operators += match self.current_tok.clone().unwrap().type_ {
                    TokType::Mult => "*",
                    TokType::Div  => "/",
                    TokType::FDiv => "f",
                    TokType::Mod  => "%",
                    _ => unreachable!()
                };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                }
                children.push(match self.power() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.clone().last().unwrap().pos_end;
            return Ok(ASTNode::new(ASTNodeType::ArithOp(operators.chars().collect::<Vec<char>>()),
                                   children,
                                   pos_start,
                                   pos_end));
        }

        Ok(node)
    }

    fn power(&mut self) -> Result<ASTNode, Error> {
        let node = match self.factor() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        if self.current_tok.clone().unwrap().type_ == TokType::Pow {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TokType::Pow {
                operators += "p";
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
                }
                children.push(match self.factor() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.clone().last().unwrap().pos_end;
            return Ok(ASTNode::new(ASTNodeType::ArithOp(operators.chars().collect::<Vec<char>>()),
                                   children,
                                   pos_start,
                                   pos_end));
        }

        Ok(node)
    }

    fn factor(&mut self) -> Result<ASTNode, Error> {
        if self.current_tok.is_none() {
            return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0305));
        }
        let tok = self.current_tok.clone().unwrap();
        let tok_type = tok.type_;
        let pos_start = tok.pos_start;
        if tok_type == TokType::Plus || tok_type == TokType::Minus || tok_type == TokType::Not {
            let node_type = match tok_type {
                TokType::Plus => ASTNodeType::UnaryPlus,
                TokType::Minus => ASTNodeType::UnaryMinus,
                TokType::Not => ASTNodeType::UnaryNot,
                _ => unreachable!()
            };
            self.next();
            let out = match self.factor() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(ASTNode::new(node_type, vec![out.clone()], pos_start, out.pos_end));
        }

        self.call()
    }

    fn call(&mut self) -> Result<ASTNode, Error> {
        let mut out: ASTNode;
        if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TokType::LParen {
            let pos_start = self.current_tok.clone().unwrap().pos_start;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0308));
            }
            out = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TokType::RParen {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), 0308));
            }
            out.pos_start = pos_start;
            out.pos_end = self.current_tok.clone().unwrap().pos_end;
            self.next();
        } else {
            out = match self.atom() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
        }

        return if self.current_tok.is_none() {
            Ok(out)
        } else {
            // as(0.19)
            if self.current_tok.clone().unwrap() == TokType::Keyword(0, 19) {
                self.next();
                return if self.current_tok.is_none() {
                    Err(Error::new(self.previous_end, self.previous_end.advance(), 0309))
                } else if let TokType::Keyword(1, value) = self.current_tok.clone().unwrap().type_ {
                    let pos_end = self.current_tok.clone().unwrap().pos_end;
                    self.next();
                    Ok(ASTNode::new(ASTNodeType::As(value), vec![out.clone()], out.pos_start, pos_end))
                } else {
                    Err(Error::new(self.previous_end, self.previous_end.advance(), 0309))
                };
            } else {
                Ok(out)
            }
        };
    }

    fn atom(&mut self) -> Result<ASTNode, Error> {
        let tok = self.current_tok.clone().unwrap().clone();
        let pos_start = tok.pos_start.clone();

        let out = if tok ==  TokType::Keyword(0, 20) {
            self.next();
            ASTNode::new_v(ASTNodeType::Bool(true), pos_start, tok.pos_end)
        } else if tok == TokType::Keyword(0, 21) {
            self.next();
            ASTNode::new_v(ASTNodeType::Bool(false), pos_start, tok.pos_end)
        } else {
            match tok.type_ {
                TokType::Label(v) => {
                    self.next();
                    match self.nameable_methods(Some(v)) {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    }
                }
                TokType::Int(v) => {
                    self.next();
                    ASTNode::new_v(ASTNodeType::Int(v as i64), pos_start, tok.pos_end)
                },
                TokType::Float(v) => {
                    self.next();
                    ASTNode::new_v(ASTNodeType::Float(v), pos_start, tok.pos_end)
                },
                TokType::String(v) => {
                    self.next();
                    ASTNode::new_v(ASTNodeType::String(v), pos_start, tok.pos_end)
                },
                TokType::Identifier(lang, v) => {
                    let keyed_name = format!("{}:{}", lang, v);
                    let mut current_scope_index = self.symbol_tables.last().unwrap().scope_index.clone();
                    let mut st_index = 0;
                    let mut var_index = 0;
                    let mut found = false;
                    while let Some(i) = current_scope_index.pop() {
                        if let Some(pos) = self.symbol_tables.get(i).unwrap().find(&keyed_name) {
                            var_index = pos;
                            found = true;
                            self.symbol_tables[i].found(var_index);
                            break
                        }
                        st_index += 1;
                    }
                    if !found {
                        return Err(Error::new(pos_start, tok.pos_end, 0000))
                    }
                    self.next();
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TokType::Newline {
                        return Ok(ASTNode::new_v(
                            ASTNodeType::VarAccess(st_index, var_index),
                            pos_start,
                            tok.pos_end
                        ))
                    }
                    // Check for variable reassignments
                    let t = match self.current_tok.clone().unwrap().type_ {
                        TokType::Set | TokType::SetPlus | TokType::SetMinus | TokType::SetMult | TokType::SetDiv | TokType::SetPow | TokType::SetFDiv | TokType::SetMod => {
                            self.current_tok.clone().unwrap().type_
                        }
                        _ => return Ok(ASTNode::new_v(
                            ASTNodeType::VarAccess(st_index, var_index),
                            pos_start,
                            tok.pos_end
                        ))
                    };
                    self.next();
                    return match self.expr() {
                        Ok(e) => {
                            let e_pos_start = e.pos_start.clone();
                            let pos_end = e.pos_end.clone();
                            Ok(ASTNode::new(
                                ASTNodeType::VarReassign(st_index, var_index),
                                vec![match t {
                                    TokType::Set => e,
                                    _ => ASTNode::new(
                                        ASTNodeType::ArithOp(vec![
                                            match t {
                                                TokType::SetPlus => '+',
                                                TokType::SetMinus => '-',
                                                TokType::SetMult => '*',
                                                TokType::SetDiv => '/',
                                                TokType::SetPow => 'p',
                                                TokType::SetFDiv => 'f',
                                                TokType::SetMod => '%',
                                                _ => unreachable!()
                                            }
                                        ]),
                                        vec![
                                            ASTNode::new_v(
                                                ASTNodeType::VarAccess(st_index, var_index),
                                                Position::new(), Position::new()
                                            ), e
                                        ],
                                        e_pos_start, pos_end
                                    )
                                }],
                                pos_start, pos_end
                            ))
                        }
                        Err(e) => Err(e)
                    }
                },
                _ => {
                    match self.nameable_methods(None) {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    }
                }
            }
        };
        Ok(out)
    }

    fn nameable_methods(&mut self, loop_value: Option<String>) -> Result<ASTNode, Error> {
        let tok = self.current_tok.clone().unwrap().clone();
        let pos_start = tok.pos_start.clone();

        // if(0.3) elif(0.5) else(0.4)
        if tok == TokType::Keyword(0, 3) {
            self.new_scope(loop_value.clone());
            let mut children = vec![];

            // if(0.3) statement
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0302));
            }
            children.push(match self.conditional_suite_generator() {
                Ok(n) => n,
                Err(e) => return Err(e)
            });
            
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TokType::Newline {
                self.next()
            }

            // Check for elif(0.5) statements
            if self.current_tok.is_some() && self.current_tok.clone().unwrap() == TokType::Keyword(0, 5) {
                let mut elif_exprs = vec![];
                while self.current_tok.is_some() && self.current_tok.clone().unwrap() == TokType::Keyword(0, 5) {
                    self.next();
                    if self.current_tok.is_none() {
                        return Err(Error::new(self.previous_end, self.previous_end.advance(), 0303));
                    }
                    elif_exprs.push(match self.conditional_suite_generator() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    })
                }
                children.extend(elif_exprs);
                if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    self.next()
                }
            }
            
            // else(0.4)
            if self.current_tok.is_some() && self.current_tok.clone().unwrap() == TokType::Keyword(0, 4) {
                let else_pos_start = self.current_tok.clone().unwrap().pos_start;
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0302));
                }
                if self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    self.next()
                }
                if self.current_tok.clone().unwrap().type_ != TokType::LParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0302));
                }
                self.next();
                if self.current_tok.clone().unwrap().type_ == TokType::Newline {
                    self.next()
                }
                let mut out = vec![];
                while self.current_tok.is_some() && self.current_tok.clone().unwrap() != TokType::RParenCurly {
                    out.push(match self.expr() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    });
                    self.skip_newlines();
                }
                if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::RParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308));
                }
                children.push(ASTNode::new(ASTNodeType::Else, out, else_pos_start, self.current_tok.clone().unwrap().pos_end));
                self.next();
            }

            let pos_end = children.clone().pop().unwrap().pos_end;
            // self.next();
            return Ok(ASTNode::new(ASTNodeType::If(loop_value), children, pos_start, pos_end));
        }

        // while(0.13)
        if tok == TokType::Keyword(0, 13) {
            self.next();
            let expr = match self.conditional_suite_generator() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::RParenCurly {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308));
            }
            self.next();
            return Ok(ASTNode::new(ASTNodeType::While(loop_value), vec![expr.clone()], pos_start, expr.pos_end));
        }

        // iterate(0.9)
        if tok == TokType::Keyword(0, 9) {
            let mut child_nodes = vec![];
            self.next();
            let range = match self.comp_expr() {
                Ok(n) => {
                    // Whatever this returns will have to be checked later to have a specific trait
                    // This allows for custom types to be iterated over by implementing a trait
                    n
                }
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308));
            } else if self.current_tok.clone().unwrap().type_ == TokType::Set {
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0304));
                }
                let tok = self.current_tok.clone().unwrap();
                if let TokType::Identifier(_, v) = tok.type_ {
                    child_nodes.push(ASTNode::new(ASTNodeType::VarAssign(0, v), vec![], tok.pos_start, tok.pos_end));
                    self.next();
                } else {
                    return Err(Error::new(tok.pos_start, tok.pos_end, 0304));
                }
            } else if self.current_tok.clone().unwrap().type_ != TokType::LParenCurly {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start, tok.pos_end, 0303));
            }
            self.next();
            child_nodes.push(range);
            while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
                child_nodes.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                self.skip_newlines();
            }
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308));
            } else if self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start, tok.pos_end, 0308));
            }
            let pos_end = self.current_tok.clone().unwrap().pos_end;
            self.next();
            return Ok(ASTNode::new(ASTNodeType::Iterate(loop_value), child_nodes, pos_start, pos_end));
        }

        // case(0.6)
        if tok == TokType::Keyword(0, 6) {
            self.next();
            let mut children = vec![match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            }];
            if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::LParenCurly {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0303))
            }
            self.next();
            self.skip_newlines();
            while self.current_tok.is_some() && self.current_tok.clone().unwrap() == TokType::Keyword(0, 7) {
                let condition_pos_start = self.current_tok.clone().unwrap().pos_start;
                self.next();
                let mut condition = match self.atom() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                };
                condition.pos_start = condition_pos_start;
                if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::LParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0303))
                }
                while self.current_tok.is_some() && self.current_tok.clone().unwrap() != TokType::RParenCurly {
                    self.next();
                    self.skip_newlines();
                    condition.child_nodes.push(match self.statement() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    });
                }
                if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308))
                }
                self.next();
                self.skip_newlines();
                children.push(condition);
            }
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308))
            }
            if self.current_tok.clone().unwrap() == TokType::Keyword(0, 8) {
                let mut default = ASTNode::new_v(ASTNodeType::Else, self.current_tok.clone().unwrap().pos_start, Position::new());
                self.next();
                self.skip_newlines();
                if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TokType::LParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0303))
                }
                while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
                    self.next();
                    self.skip_newlines();
                    default.child_nodes.push(match self.statement() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    });
                }
                if self.current_tok.is_none() || self.current_tok.clone().unwrap() != TokType::RParenCurly {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308))
                }
                default.pos_end = self.current_tok.clone().unwrap().pos_end;
                self.next();
                self.skip_newlines();
                children.push(default);
            }
            if self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308))
            } else if self.current_tok.clone().unwrap() == TokType::Keyword(0, 8) {

            }
            let pos_end = self.current_tok.clone().unwrap().pos_end;
            self.next();
            return Ok(ASTNode::new(ASTNodeType::Case(loop_value), children, pos_start, pos_end))
        }

        Err(Error::new(pos_start, tok.pos_end, 0305))
    }

    fn conditional_suite_generator(&mut self) -> Result<ASTNode, Error> {
        let mut expr = match self.expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), 0303));
        }
        if self.current_tok.clone().unwrap().type_ == TokType::Newline {
            self.next()
        }
        if self.current_tok.clone().unwrap().type_ != TokType::LParenCurly {
            return Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, 0303));
        }
        self.next();
        if self.current_tok.is_none() {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), 0305));
        }

        let mut out: Vec<ASTNode> = vec![];
        self.skip_newlines();
        while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
            out.push(match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            });
            self.skip_newlines();
        }
        if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TokType::RParenCurly {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), 0308));
        }
        self.next();
        expr.child_nodes.extend(out);
        Ok(expr)
    }
}
