use crate::tokens::*;
use crate::bases::*;
use crate::err_wrn::*;
use lang::keywords::Keywords;
use crate::nodes::{ASTNode, ASTNodeType};
use lang::get_associated_keywords;
use std::collections::HashMap;

pub struct Lexer {
    split_text: Vec<char>,
    current_pos: Position,
    char_index: usize,
    current_char: char,
    pub(crate) keywords: Keywords<'static>,
    pub(crate) keyword_code: String,
}

impl Lexer {
    pub fn new(full_text: String, keywords: Keywords<'static>, keyword_code: String) -> Lexer {
        return Lexer {
            split_text: full_text.chars().collect(),
            current_pos: Position::new(),
            char_index: 0,
            current_char: full_text.chars().nth(0).unwrap(),
            keywords,
            keyword_code,
        };
    }

    fn advance(&mut self) {
        self.current_pos = self.current_pos.advance();
        self.char_index += 1;
        if self.char_index >= self.split_text.len() {
            self.current_char = char::from(0);
        } else {
            self.current_char = self.split_text[self.char_index]
        }
    }

    // TODO: Change the return type to either an error or token
    // TODO: Change the return types of referenced functions to be (ErrorOrNot, Option<Token>, Option<Error>)
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
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
                        tokens.push(Token::new(TT_NEWLINE, "".into(), pos_start,
                                               self.current_pos.clone()));
                        self.current_pos.advance_ln();
                    }
                    self.advance();
                } else if self.current_char == '#' {
                    self.advance();
                    if self.current_char == '!' {
                        self.advance();
                        let mut lang_code = self.make_identifier().value;
                        match get_associated_keywords(lang_code.as_str()) {
                            Some(k) => self.keywords = k,
                            None => return Err(Error::new(pos_start,
                                                          self.current_pos.clone(),
                                                          line!() as u16,
                                                          "".to_string()))
                        }
                        self.keyword_code = lang_code;
                        self.advance();
                    } else {
                        self.skip_comment();
                    }
                } else {
                    let tok_type: u8 = match self.current_char {
                        '+' => TT_PLUS,
                        '-' => TT_MINUS,
                        '%' => TT_MOD,
                        '(' => TT_LPAREN,
                        ')' => TT_RPAREN,
                        '{' => TT_LPAREN_CURLY,
                        '}' => TT_RPAREN_CURLY,
                        ',' => TT_COMMA,
                        '[' => TT_LPAREN_SQUARE,
                        ']' => TT_RPAREN_SQUARE,
                        ';' => TT_NEWLINE,
                        '?' => TT_QUESTION_MARK,
                        '.' => TT_DOT,
                        _ => 0
                    };
                    if tok_type > 0 {
                        self.advance();
                        tokens.push(Token::new(tok_type, "".into(), pos_start,
                                               self.current_pos.clone()));
                        continue;
                    }

                    let tok = match match self.current_char {
                        '!' => self.make_not_equals(),
                        '<' => self.single_double_token('=', TT_LT, TT_LTE),
                        '>' => self.single_double_token('=', TT_GT, TT_GTE),
                        '*' => self.single_double_token('*', TT_MULT, TT_POW),
                        '/' => self.single_double_token('/', TT_DIV, TT_FDIV),
                        ':' => self.make_set(),
                        '=' => self.make_equals(),
                        '@' => self.make_loop_identifier(),
                        '\'' | '"' => self.make_string(self.current_char),
                        _ => Err(Error::new(pos_start, pos_start.clone().advance(), line!() as u16, String::new()))
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

    // Character parsing functions
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

        return Token::new(has_dot as u8, value, pos_start,
                          self.current_pos.clone());
    }

    fn make_identifier(&mut self) -> Token {
        let pos_start = self.current_pos.clone();
        let mut keyword = self.current_char.to_string();
        self.advance();

        while (self.current_char.is_alphanumeric() || self.current_char == '_') &&
            self.char_index < self.split_text.len() {
            keyword.push(self.current_char);
            self.advance();
        }

        let (value, tok_type) = match self.keywords.contains(&keyword) {
            Some(val) => (val, TT_KEYWORD),
            None => (keyword, TT_IDENTIFIER)
        };

        return Token::new(tok_type, value, pos_start, self.current_pos.clone());
    }

    fn make_loop_identifier(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        Ok(Token::new(TT_AT, self.make_identifier().value, pos_start,
                      self.current_pos.clone()))
    }

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
        Ok(Token::new(TT_STRING, out, pos_start, self.current_pos.clone()))
    }

    fn make_not_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        if self.current_char == '=' {
            self.advance();
            return Ok(Token::new(TT_NE, "".into(), pos_start,
                                 self.current_pos.clone()));
        } else if self.current_char.is_alphabetic() || "_!".contains(self.current_char) {
            return Ok(Token::new(TT_NOT, "".into(), pos_start,
                                 self.current_pos.clone()));
        }
        return Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()));
    }

    fn make_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Ok(
                Token::new(TT_EQ, "".into(),
                           pos_start,
                           self.current_pos.clone()));
        }
        // self.advance();
        return Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()));
    }

    fn single_double_token(&mut self, second_char: char, single_type: u8, double_type: u8) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        let mut tok_type = 0;
        if self.current_char == second_char {
            self.advance();
            tok_type = double_type;
        } else {
            tok_type = single_type;
        }
        return Ok(Token::new(tok_type, "".into(), pos_start,
                             self.current_pos.clone()));
    }

    fn make_set(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        let mut tok_type = 0u8;
        if !":>".contains(self.current_char) {
            tok_type += match self.current_char {
                '+' => {
                    self.advance();
                    TT_PLUS
                }
                '-' => {
                    self.advance();
                    TT_MINUS
                }
                '%' => {
                    self.advance();
                    TT_MOD
                }
                '*' => {
                    self.advance();
                    match self.current_char {
                        '*' => {
                            self.advance();
                            TT_POW
                        }
                        ':' => TT_MULT,
                        '>' => TT_MULT,
                        _ => return Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()))
                    }
                }
                '/' => {
                    self.advance();
                    match self.current_char {
                        '/' => {
                            self.advance();
                            TT_FDIV
                        }
                        ':' => TT_DIV,
                        '>' => TT_DIV,
                        _ => return Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()))
                    }
                }
                _ => return Ok(Token::new(TT_COLON, "".into(), pos_start, self.current_pos.clone()))
            } - 2;
        }
        tok_type += match self.current_char {
            ':' => 32,
            '>' => 40,
            _ => {
                self.advance();
                return Ok(Token::new(TT_COLON, "".into(), pos_start,
                                     self.current_pos.clone()));
            }
        };
        self.advance();
        return Ok(Token::new(tok_type, "".into(), pos_start,
                             self.current_pos.clone()));
    }

    fn skip_comment(&mut self) {
        if self.current_char == '#' {
            while self.current_char != '\n' || self.char_index + 1 != self.split_text.len() {
                self.advance();
            }
        } else {
            while self.current_char != '#' && self.char_index + 1 != self.split_text.len() {
                self.advance();
            }
            self.advance();
        }
    }
}

pub struct Parser {
    to_process: Vec<Token>,
    intermediate: Vec<Token>,
    current_tok: Option<Token>,
    previous_end: Position,
    safe: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens;
        tokens.reverse();
        let current_tok = tokens.clone().pop().unwrap();
        return Parser {
            to_process: tokens,
            intermediate: Vec::new(),
            current_tok: Some(current_tok.clone()),
            previous_end: current_tok.pos_start,
            safe: false,
        };
    }

    // Navigating tokens functions
    fn next(&mut self) {
        // Goes to the next token and adds the previous one to the intermediate vector
        // Returns true if more tokens are left and false otherwise
        if self.current_tok.is_none() {
            return;
        } else if self.safe {
            self.intermediate.push(self.current_tok.clone().unwrap());
        }
        self.current_tok = self.to_process.pop();
    }

    fn finalise(&mut self) {
        // Called when the previously processed tokens can safely be moved into the processed
        // vector and the intermediate vector can be cleared
        self.intermediate.clear();
        self.safe = false;
    }

    fn put_back(&mut self) {
        self.intermediate.push(self.current_tok.clone().unwrap());

        self.intermediate.reverse();
        self.to_process.append(&mut self.intermediate);
        self.intermediate.clear();
        self.safe = false;
        self.next();
    }

    // General useful functions
    fn skip_newlines(&mut self) {
        while self.current_tok.is_some() {
            if self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                self.next()
            } else {
                return;
            }
        }
    }

    // Parsing tokens
    pub fn parse(&mut self) -> Result<Vec<ASTNode>, Error> {
        let mut out: Vec<ASTNode> = Vec::new();

        // Loop through tokens to create an ast
        loop {
            self.next();
            if self.current_tok.is_none() { break; }

            self.skip_newlines();
            if self.current_tok.is_none() { break; }

            match self.statement() {
                Ok(AST) => out.push(AST),
                Err(error) => return Err(error)
            }
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TT_NEWLINE {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start,
                                      tok.pos_end,
                                      line!() as u16,
                                      "".to_string(),
                ));
            }
        }
        return Ok(out);
    }

    fn statement(&mut self) -> Result<ASTNode, Error> {
        let pos_start = self.current_tok.clone().unwrap().pos_start.clone();
        let mut pos_end = self.current_tok.clone().unwrap().pos_end.clone();
        let tok = self.current_tok.clone().unwrap();

        // return keyword
        if tok.matches(TT_KEYWORD, "0.15") {
            self.next();
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                return Ok(ASTNode::new(ASTNodeType::Return(false),
                                       Vec::new(),
                                       pos_start,
                                       pos_end));
            } else if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.3") {
                self.safe = true;
                self.next();
                println!("{:?}", self.intermediate);
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                }
                let conditional = match self.comp_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                };
                return if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                    self.safe = false;
                    self.finalise();
                    Ok(ASTNode::new(ASTNodeType::Return(true),
                                    vec![conditional],
                                    pos_start,
                                    pos_end))
                } else if self.current_tok.clone().unwrap().type_ != TT_LPAREN_CURLY {
                    Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()))
                } else {
                    println!("{:?}", self.intermediate);
                    self.put_back();
                    println!("{:?}", self.current_tok.clone().unwrap());
                    let ret_value = match self.expr() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    };
                    pos_end = ret_value.clone().pos_end;
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                        Ok(ASTNode::new(ASTNodeType::Return(false),
                                        vec![ret_value],
                                        pos_start,
                                        pos_end))
                    } else if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.3") {
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
                        Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()))
                    }
                };
            }
            let expr = match self.expr() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };

            return if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                pos_end = expr.pos_end.clone();
                Ok(ASTNode::new(ASTNodeType::Return(false),
                                Vec::from([expr]),
                                pos_start,
                                pos_end))
            } else if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.3") {
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
                Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()))
            };
        }

        // continue(0.16) and break(0.17) keywords
        if tok.matches(TT_KEYWORD, "0.16") || tok.matches(TT_KEYWORD, "0.17") {
            let pos_start = self.current_tok.clone().unwrap().pos_start;
            let mut pos_end = self.current_tok.clone().unwrap().pos_end;
            let mut out: ASTNode;

            self.next();
            if self.current_tok.is_some() {
                if self.current_tok.clone().unwrap().type_ == TT_AT {
                    out = ASTNode::new_v(match &self.current_tok.clone().unwrap().value as &str {
                        "0.16" => ASTNodeType::Continue,
                        "0.17" => ASTNodeType::Break,
                        _ => unreachable!()
                    }(Some(self.current_tok.clone().unwrap().value)), pos_start, self.current_tok.clone().unwrap().pos_end,
                    );
                } else {
                    out = ASTNode::new_v(match &self.current_tok.clone().unwrap().value as &str {
                        "0.16" => ASTNodeType::Continue,
                        "0.17" => ASTNodeType::Break,
                        _ => unreachable!()
                    }(None), pos_start, pos_end,
                    );
                }
            } else {
                out = ASTNode::new_v(match &self.current_tok.clone().unwrap().value as &str {
                    "0.16" => ASTNodeType::Continue,
                    "0.17" => ASTNodeType::Break,
                    _ => unreachable!()
                }(None), pos_start, pos_end,
                );
            }
            if self.current_tok.is_some() {
                if !self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.3") {
                    return Err(Error::new(pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()));
                }
                self.next();
                out.child_nodes = vec![match self.comp_expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                }];
                if self.current_tok.is_some() {
                    return Err(Error::new(pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()));
                }
            }
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                return Ok(out);
            }
            if self.current_tok.clone().unwrap().type_ == TT_AT {
                return Err(Error::new(pos_start, pos_end, line!() as u16, String::new()));
            }
        }

        self.expr()
    }

    fn expr(&mut self) -> Result<ASTNode, Error> {
        let mut tok = self.current_tok.clone().unwrap();
        fn default_values(index: u8) -> ASTNodeType {
            match index {
                0 => ASTNodeType::Int(0),
                1 => ASTNodeType::Float(0.),
                2 => ASTNodeType::Bool(false),
                3 => ASTNodeType::String(String::new()),
                4 => ASTNodeType::List,
                _ => unreachable!()
            }
        }

        let pos_start = tok.pos_start.clone();
        let mut pos_end = tok.pos_end.clone();

        // check for new variable assignments
        if tok.matches_list(1) {
            let var_type = tok.value.get(2..).unwrap().parse::<u8>().unwrap();
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance().clone(), line!() as u16, String::new()));
            };
            tok = self.current_tok.clone().unwrap();
            if tok.type_ != TT_IDENTIFIER {
                return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
            }
            let var_name = tok.value.clone();
            pos_end = tok.pos_end.clone();
            let expr: ASTNode;

            self.next();
            if self.current_tok.is_none() {
                expr = ASTNode::new(default_values(var_type),
                                    Vec::new(),
                                    Position::new(),
                                    Position::new());
                return Ok(ASTNode::new(ASTNodeType::VarAssign(false, var_type, var_name),
                                       Vec::from([expr]),
                                       pos_start,
                                       pos_end));
            };

            let mut ret = false;
            let mut pos_end = self.current_tok.clone().unwrap().pos_end;
            match self.current_tok.clone().unwrap().type_ {
                TT_SET => {
                    self.next();
                    if self.current_tok.is_none() {
                        return Err(Error::new(self.previous_end, self.previous_end.advance().clone(), line!() as u16, String::new()));
                    } else {
                        expr = match self.expr() {
                            Ok(ast) => ast,
                            Err(e) => return Err(e)
                        };
                        pos_end = expr.pos_end;
                    }
                }
                TT_SET_RET => {
                    ret = true;
                    pos_end = self.current_tok.clone().unwrap().pos_end.clone();
                    self.next();
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                        expr = ASTNode::new(default_values(var_type),
                                            Vec::new(),
                                            Position::new(),
                                            Position::new());
                        if self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
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
                TT_NEWLINE => {
                    todo!();
                    panic!()
                }
                _ => return Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_start, line!() as u16, String::new()))
            }
            return Ok(ASTNode::new(ASTNodeType::VarAssign(ret, var_type, var_name),
                                   Vec::from([expr]),
                                   pos_start,
                                   pos_end));
        };

        // TODO: static(previously silent) variable assignments

        // Variable access and reassignment
        if tok.type_ == TT_IDENTIFIER {
            let var_name = tok.value.clone();
            self.safe = true;
            self.next();
            if self.current_tok.is_none() {
                self.safe = false;
                return Ok(ASTNode::new(ASTNodeType::VarAccess(var_name),
                                       Vec::new(),
                                       pos_start,
                                       pos_end));
            } else {
                self.put_back();
            }
        }

        let node = match self.comp_expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        // 0.0 is 'and', 0.1 is 'or'
        if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") || self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.1") {
            let mut children = vec![node];
            let mut operators = String::new();
            while self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") || self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.1") {
                operators += if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") { "&" } else { "|" };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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

        if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TT_QUESTION_MARK {
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
            }
            let mut children = vec![node.clone()];
            if self.current_tok.clone().unwrap().type_ == TT_COLON {
                // Only the false option given
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
                } else if self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                    let tok = self.current_tok.clone().unwrap();
                    return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
                }

                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });

                pos_end = children.clone().pop().unwrap().pos_end;
                return Ok(ASTNode::new(ASTNodeType::Ternary(false, true), children, node.pos_start, pos_end));
            } else {
                // true option definitely given
                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                pos_end = children.clone().pop().unwrap().pos_end;
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
                } else if self.current_tok.clone().unwrap().type_ != TT_COLON {
                    let tok = self.current_tok.clone().unwrap();
                    return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
                }
                self.next();
                if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                    // Only the true option given
                    return Ok(ASTNode::new(ASTNodeType::Ternary(true, false), children, pos_start, pos_end));
                }
                children.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                pos_end = children.clone().pop().unwrap().pos_end;
                return Ok(ASTNode::new(ASTNodeType::Ternary(true, true), children, pos_start, pos_end));
            }
        }

        Ok(node)
    }

    fn comp_expr(&mut self) -> Result<ASTNode, Error> {
        // 0.2 is 'not'
        if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.2") {
            let pos_start = self.current_tok.clone().unwrap().pos_start;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            }
            let node = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            return Ok(ASTNode::new(ASTNodeType::UnaryNot,
                                   vec![node.clone()],
                                   pos_start,
                                   node.pos_end
            ));
        }

        let node = match self.arith_expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Ok(node);
        }

        if 23 < self.current_tok.clone().unwrap().type_ && self.current_tok.clone().unwrap().type_ < 30 {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while 23 < self.current_tok.clone().unwrap().type_ && self.current_tok.clone().unwrap().type_ < 30 {
                operators += match self.current_tok.clone().unwrap().type_ {
                    24 => "e", // ==
                    25 => "n", // !=
                    26 => "l", // <
                    27 => "g", // >
                    28 => "L", // <=
                    29 => "G", // >=
                    _ => panic!("Why are here!")
                };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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
        } else if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.10") {
            let start_node = node;
            let pos_start = start_node.clone().pos_start;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
            }
            let end_node = match self.arith_expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() || !self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.12") {
                return Ok(ASTNode::new(ASTNodeType::Range, vec![start_node, end_node.clone()], pos_start, end_node.pos_end));
            }
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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

        if self.current_tok.clone().unwrap().type_ == TT_PLUS || self.current_tok.clone().unwrap().type_ == TT_MINUS {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TT_PLUS || self.current_tok.clone().unwrap().type_ == TT_MINUS {
                operators += if self.current_tok.clone().unwrap().type_ == TT_PLUS { "+" } else { "-" };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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

        if self.current_tok.clone().unwrap().type_ == TT_MULT || self.current_tok.clone().unwrap().type_ == TT_DIV || self.current_tok.clone().unwrap().type_ == TT_FDIV || self.current_tok.clone().unwrap().type_ == TT_MOD {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TT_MULT || self.current_tok.clone().unwrap().type_ == TT_DIV || self.current_tok.clone().unwrap().type_ == TT_FDIV || self.current_tok.clone().unwrap().type_ == TT_MOD {
                operators += match self.current_tok.clone().unwrap().type_ {
                    TT_MULT => "*",
                    TT_DIV => "/",
                    TT_FDIV => "f",
                    TT_MOD => "%",
                    _ => panic!("Why are you here")
                };
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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

        if self.current_tok.clone().unwrap().type_ == TT_POW {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TT_POW {
                operators += "p";
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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
            return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
        }
        let tok = self.current_tok.clone().unwrap();
        let tok_type = tok.type_;
        let pos_start = tok.pos_start;
        if tok_type == TT_PLUS || tok_type == TT_MINUS || tok_type == TT_NOT {
            let node_type = match tok_type {
                TT_PLUS => ASTNodeType::UnaryPlus,
                TT_MINUS => ASTNodeType::UnaryMinus,
                TT_NOT => ASTNodeType::UnaryNot,
                _ => { panic!("why are here?!") }
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
        if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ == TT_LPAREN {
            let pos_start = self.current_tok.clone().unwrap().pos_start;
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
            }
            out = match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TT_RPAREN {
                return Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
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
            if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.19") {
                self.next();
                return if self.current_tok.is_none() {
                    Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, "".to_string()))
                } else if self.current_tok.clone().unwrap().type_ == TT_KEYWORD && self.current_tok.clone().unwrap().value.chars().nth(0).unwrap() == '1' {
                    let value = self.current_tok.clone().unwrap().value.get(2..).unwrap().to_string().parse::<u8>().unwrap();
                    let pos_end = self.current_tok.clone().unwrap().pos_end;
                    self.next();
                    Ok(ASTNode::new(ASTNodeType::As(value), vec![out.clone()], out.pos_start, pos_end))
                } else {
                    Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, "".to_string()))
                };
            } else {
                Ok(out)
            }
        };
    }

    fn atom(&mut self) -> Result<ASTNode, Error> {
        let tok = self.current_tok.clone().unwrap().clone();
        let pos_start = tok.pos_start.clone();

        let out = if tok.matches(TT_KEYWORD, "0.20") {
            ASTNode::new_v(ASTNodeType::Bool(true), pos_start, tok.pos_end)
        } else if tok.matches(TT_KEYWORD, "0.21") {
            ASTNode::new_v(ASTNodeType::Bool(false), pos_start, tok.pos_end)
        } else {
            match tok.type_ {
                TT_AT => {
                    self.next();
                    match self.nameable_methods(Some(tok.value)) {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    }
                }
                TT_INT => ASTNode::new_v(ASTNodeType::Int(tok.value.parse::<i64>().unwrap()), pos_start, tok.pos_end),
                TT_FLOAT => ASTNode::new_v(ASTNodeType::Float(tok.value.parse::<f64>().unwrap()), pos_start, tok.pos_end),
                TT_STRING => ASTNode::new_v(ASTNodeType::String(tok.value), pos_start, tok.pos_end),
                TT_IDENTIFIER => ASTNode::new_v(ASTNodeType::VarAccess(tok.value), pos_start, tok.pos_end),
                _ => {
                    match self.nameable_methods(None) {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    }
                }
            }
        };
        self.next();
        Ok(out)
    }

    fn nameable_methods(&mut self, loop_value: Option<String>) -> Result<ASTNode, Error> {
        let tok = self.current_tok.clone().unwrap().clone();
        let pos_start = tok.pos_start.clone();

        // if(0.3) elif(0.5) else(0.4)
        if tok.matches(TT_KEYWORD, "0.3") {
            let mut children = vec![];

            // if(0.3) statement
            self.next();
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            }
            children.push(match self.conditional_suite_generator() {
                Ok(n) => n,
                Err(e) => return Err(e)
            });
            if !(self.current_tok.clone().is_some() && self.current_tok.clone().unwrap().type_ == TT_RPAREN_CURLY) {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            }
            self.next();

            self.skip_newlines();

            // Check for elif(0.5) statements
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.5") {
                let mut elif_exprs = vec![];
                while self.current_tok.is_some() && self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.5") {
                    self.next();
                    if self.current_tok.is_none() {
                        return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                    }
                    elif_exprs.push(match self.conditional_suite_generator() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    })
                }
                children.extend(elif_exprs);
            }

            self.skip_newlines();

            // else(0.4)
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.4") {
                let else_pos_start = self.current_tok.clone().unwrap().pos_start;
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                }
                if self.current_tok.clone().unwrap().type_ != TT_LPAREN_CURLY {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                }
                self.next();
                let mut out = vec![];
                while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
                    out.push(match self.expr() {
                        Ok(n) => n,
                        Err(e) => return Err(e)
                    });
                    self.skip_newlines();
                }
                if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                }
                children.push(ASTNode::new(ASTNodeType::Else, out, else_pos_start, self.current_tok.clone().unwrap().pos_end));
            }

            let pos_end = children.clone().pop().unwrap().pos_end;
            self.next();
            return Ok(ASTNode::new(ASTNodeType::If(loop_value), children, pos_start, pos_end));
        }

        // while(0.13)
        if tok.matches(TT_KEYWORD, "0.13") {
            self.next();
            let expr = match self.conditional_suite_generator() {
                Ok(n) => n,
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            }
            self.next();
            return Ok(ASTNode::new(ASTNodeType::While(loop_value), vec![expr.clone()], pos_start, expr.pos_end));
        }

        // iterate(0.9)
        if tok.matches(TT_KEYWORD, "0.9") {
            let mut child_nodes = vec![];
            self.next();
            let range = match self.comp_expr() {
                Ok(n) => {
                    if n.node_type != ASTNodeType::Range {
                        return Err(Error::new(n.pos_start, n.pos_end, line!() as u16, String::new()));
                    }
                    n
                }
                Err(e) => return Err(e)
            };
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            } else if self.current_tok.clone().unwrap().type_ == TT_SET {
                self.next();
                if self.current_tok.is_none() {
                    return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
                }
                let tok = self.current_tok.clone().unwrap();
                if tok.type_ != TT_IDENTIFIER {
                    return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
                }
                child_nodes.push(ASTNode::new(ASTNodeType::VarAssign(false, 0, tok.value), vec![], tok.pos_start, tok.pos_end));
                self.next();
            } else if self.current_tok.clone().unwrap().type_ != TT_LPAREN_CURLY {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
            }
            self.next();
            child_nodes.push(range);
            while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
                child_nodes.push(match self.expr() {
                    Ok(n) => n,
                    Err(e) => return Err(e)
                });
                self.skip_newlines();
            }
            if self.current_tok.is_none() {
                return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            } else if self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
                let tok = self.current_tok.clone().unwrap();
                return Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
            }
            let pos_end = self.current_tok.clone().unwrap().pos_end;
            self.next();
            return Ok(ASTNode::new(ASTNodeType::Iterate(loop_value), child_nodes, pos_start, pos_end));
        }

        // case(0.6)
        if tok.matches(TT_KEYWORD, "0.6") {
            self.next();
            //TODO: keep going
        }

        Err(Error::new(pos_start, tok.pos_end, line!() as u16, String::new()))
    }

    fn conditional_suite_generator(&mut self) -> Result<ASTNode, Error> {
        let mut expr = match self.expr() {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        if self.current_tok.is_none() {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
        } else if self.current_tok.clone().unwrap().type_ != TT_LPAREN_CURLY {
            return Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_end, line!() as u16, String::new()));
        }
        self.next();
        if self.current_tok.is_none() {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
        }

        let mut out: Vec<ASTNode> = vec![];
        while self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
            out.push(match self.expr() {
                Ok(n) => n,
                Err(e) => return Err(e)
            });
            self.skip_newlines();
        }
        if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ != TT_RPAREN_CURLY {
            return Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
        }
        expr.child_nodes.extend(out);
        self.skip_newlines();
        Ok(expr)
    }
}
