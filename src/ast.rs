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
                            None => return Result::Err(Error::new(pos_start,
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
                        _ => Result::Err(Error::new(pos_start, pos_start.clone().advance(), line!() as u16, String::new()))
                    } {
                        Ok(tok) => tok,
                        Err(e) => return Result::Err(e)
                    };
                    tokens.push(tok);
                }
            }
        }
        Result::Ok(tokens)
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
        Result::Ok(Token::new(TT_AT, self.make_identifier().value, pos_start,
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
        Result::Ok(Token::new(TT_STRING, out, pos_start, self.current_pos.clone()))
    }

    fn make_not_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();
        if self.current_char == '=' {
            self.advance();
            return Result::Ok(Token::new(TT_NE, "".into(), pos_start,
                                         self.current_pos.clone()));
        } else if self.current_char.is_alphabetic() || "_!".contains(self.current_char) {
            return Result::Ok(Token::new(TT_NOT, "".into(), pos_start,
                                         self.current_pos.clone()));
        }
        return Result::Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()));
    }

    fn make_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.clone();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Result::Ok(
                Token::new(TT_EQ, "".into(),
                           pos_start,
                           self.current_pos.clone()));
        }
        // self.advance();
        return Result::Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()));
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
        return Result::Ok(Token::new(tok_type, "".into(), pos_start,
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
                        _ => return Result::Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()))
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
                        _ => return Result::Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()))
                    }
                }
                _ => return Result::Err(Error::new(pos_start, self.current_pos.clone(), line!() as u16, String::new()))
            } - 2;
        }
        tok_type += match self.current_char {
            ':' => 32,
            '>' => 40,
            _ => {
                self.advance();
                return Result::Ok(Token::new(TT_COLON, "".into(), pos_start,
                                             self.current_pos.clone()));
            }
        };
        self.advance();
        return Result::Ok(Token::new(tok_type, "".into(), pos_start,
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
            self.intermediate.append(&mut vec![self.current_tok.clone().unwrap()]);
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
        self.intermediate.reverse();
        self.intermediate.push(self.current_tok.clone().unwrap());
        self.intermediate.reverse();

        self.to_process.append(&mut self.intermediate);
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
                Err(error) => return Result::Err(error)
            }
            if self.current_tok.is_some() && self.current_tok.clone().unwrap().type_ != TT_NEWLINE {
                let tok = self.current_tok.clone().unwrap();
                return Result::Err(Error::new(tok.pos_start,
                                              tok.pos_end,
                                              line!() as u16,
                                              "".to_string(),
                ));
            }
        }
        return Result::Ok(out);
    }

    fn statement(&mut self) -> Result<ASTNode, Error> {
        let pos_start = self.current_tok.clone().unwrap().pos_start.clone();
        let mut pos_end = self.current_tok.clone().unwrap().pos_end.clone();
        let tok = self.current_tok.clone().unwrap();

        // return keyword
        if tok.matches(TT_KEYWORD, "0.15") {
            self.next();
            if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                return Result::Ok(ASTNode::new(ASTNodeType::Return,
                                               Vec::new(),
                                               pos_start,
                                               pos_end,
                                               None));
            }

            let expr = match self.expr() {
                Ok(expr) => expr,
                Err(e) => return Result::Err(e)
            };

            pos_end = expr.pos_end.clone();
            return Result::Ok(ASTNode::new(ASTNodeType::Return,
                                           Vec::from([expr]),
                                           pos_start,
                                           pos_end,
                                           None));
        }

        // continue(0.16) and break(0.17) keywords
        if tok.matches(TT_KEYWORD, "0.16") || tok.matches(TT_KEYWORD, "0.17") {
            let node_type = match &self.current_tok.clone().unwrap().value as &str {
                "0.16" => ASTNodeType::Continue,
                "0.17" => ASTNodeType::Break,
                _ => { panic!("Why are you here?!") }
            };

            self.next();
            if self.current_tok.is_some() {
                // TODO: check for loop identifiers here and process appropriately
                return Result::Err(Error::new(pos_start, self.current_tok.clone().unwrap().pos_end, 0u16, String::new()));
            }
            if self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                return Result::Ok(ASTNode::new(node_type,
                                               Vec::new(),
                                               pos_start,
                                               pos_end,
                                               None));
            }
            if self.current_tok.clone().unwrap().type_ == TT_AT {
                return Result::Err(Error::new(pos_start, pos_end, line!() as u16, String::new()));
            }
        }

        self.expr()
    }

    fn expr(&mut self) -> Result<ASTNode, Error> {
        let mut tok = self.current_tok.clone().unwrap();
        fn default_values(index: u8) -> (ASTNodeType, Option<String>) {
            match index {
                0 => (ASTNodeType::Int, Some("0".to_string())),
                1 => (ASTNodeType::Float, Some("0".to_string())),
                2 => (ASTNodeType::Bool, Some("0".to_string())),
                3 => (ASTNodeType::String, Some("".to_string())),
                4 => (ASTNodeType::List, None),
                _ => panic!()
            }
        }

        let pos_start = tok.pos_start.clone();
        let mut pos_end = tok.pos_end.clone();

        // check for new variable assignments
        if tok.matches_list(1) {
            let var_type = tok.value.get(2..).unwrap().parse::<u8>().unwrap();
            self.next();
            if self.current_tok.is_none() {
                return Result::Err(Error::new(self.previous_end, self.previous_end.advance().clone(), line!() as u16, String::new()));
            };
            tok = self.current_tok.clone().unwrap();
            if tok.type_ != TT_IDENTIFIER {
                return Result::Err(Error::new(tok.pos_start, tok.pos_end, line!() as u16, String::new()));
            }
            let var_name = tok.value.clone();
            pos_end = tok.pos_end.clone();
            let expr: ASTNode;

            self.next();
            if self.current_tok.is_none() {
                let (node_type, value) = default_values(var_type);
                expr = ASTNode::new(node_type,
                                    Vec::new(),
                                    Position::new(),
                                    Position::new(),
                                    value);
                return Result::Ok(ASTNode::new(ASTNodeType::VarAssign,
                                               Vec::from([expr]),
                                               pos_start,
                                               pos_end,
                                               Some(format!("0{}{}",
                                                            var_type,
                                                            var_name))));
            };

            let mut ret = false;
            let mut pos_end = self.current_tok.clone().unwrap().pos_end;
            match self.current_tok.clone().unwrap().type_ {
                TT_SET => {
                    self.next();
                    if self.current_tok.is_none() {
                        return Result::Err(Error::new(self.previous_end, self.previous_end.advance().clone(), line!() as u16, String::new()));
                    } else {
                        expr = match self.expr() {
                            Ok(ast) => ast,
                            Err(e) => return Result::Err(e)
                        };
                        pos_end = expr.pos_end;
                    }
                }
                TT_SET_RET => {
                    ret = true;
                    pos_end = self.current_tok.clone().unwrap().pos_end.clone();
                    self.next();
                    if self.current_tok.is_none() || self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                        let (node_type, value) = default_values(var_type);
                        expr = ASTNode::new(node_type,
                                            Vec::new(),
                                            Position::new(),
                                            Position::new(),
                                            value);
                        if self.current_tok.clone().unwrap().type_ == TT_NEWLINE {
                            self.next();
                        }
                    } else {
                        expr = match self.expr() {
                            Ok(ast) => ast,
                            Err(e) => return Result::Err(e)
                        };
                        pos_end = expr.pos_end;
                    }
                }
                TT_NEWLINE => {
                    todo!();
                    panic!()
                }
                _ => return Result::Err(Error::new(self.current_tok.clone().unwrap().pos_start, self.current_tok.clone().unwrap().pos_start, line!() as u16, String::new()))
            }
            return Result::Ok(ASTNode::new(ASTNodeType::VarAssign,
                                           Vec::from([expr]),
                                           pos_start,
                                           pos_end,
                                           Some(format!("{}{}{}",
                                                        ret as i8,
                                                        var_type,
                                                        var_name))));
        };

        // TODO: static(previously silent) variable assignments

        // Variable access and reassignment
        if self.current_tok.clone().unwrap().type_ == TT_IDENTIFIER {
            let var_name = self.current_tok.clone().unwrap().value.clone();
            self.next();
            if self.current_tok.is_none() {
                return Result::Ok(ASTNode::new(ASTNodeType::VarAccess,
                                               Vec::new(),
                                               pos_start,
                                               pos_end,
                                               Some(var_name)));
            }
        }

        return self.comp_expr();
    }

    fn comp_expr(&mut self) -> Result<ASTNode, Error> {
        // 0.2 is 'not'
        if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.2") {
            let op_tok = self.current_tok.clone().unwrap().clone();
            self.next();
            if self.current_tok.is_none() {
                return Result::Err(Error::new(self.previous_end, self.previous_end.advance(), line!() as u16, String::new()));
            }
            let node = match self.comp_expr() {
                Ok(n) => n,
                Err(e) => return Result::Err(e)
            };
            return Result::Ok(ASTNode::new(ASTNodeType::UnaryNot,
                                           vec![node.clone()],
                                           op_tok.pos_start,
                                           node.pos_end,
                                           None,
            ));
        }

        let node = match self.arith_expr() {
            Ok(n) => n,
            Err(e) => return Result::Err(e)
        };
        if self.current_tok.is_none() {
            return Result::Ok(node);
        }
        let mut operators = String::new();
        let pos_start = node.pos_start.clone();

        // 0.0 is 'and', 0.1 is 'or'
        if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") || self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.1") {
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") || self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.1") {
                operators += if self.current_tok.clone().unwrap().matches(TT_KEYWORD, "0.0") { "&" } else { "|" };
                children.push(match self.arith_expr() {
                    Ok(n) => n,
                    Err(e) => return Result::Err(e)
                });
            }
            let pos_end = children.last().unwrap().clone().pos_end;
            return Result::Ok(ASTNode::new(ASTNodeType::CompOp,
                                           children,
                                           pos_start,
                                           pos_end,
                                           Some(operators)));
        }

        Result::Ok(node)
    }

    fn arith_expr(&mut self) -> Result<ASTNode, Error> {
        let node = match self.term() {
            Ok(n) => n,
            Err(e) => return Result::Err(e)
        };
        if self.current_tok.is_none() {
            return Result::Ok(node);
        }

        if self.current_tok.clone().unwrap().type_ == TT_PLUS || self.current_tok.clone().unwrap().type_ == TT_MINUS {
            let mut operators = String::new();
            let pos_start = node.pos_start.clone();
            let mut children = vec![node];
            while self.current_tok.clone().unwrap().type_ == TT_PLUS || self.current_tok.clone().unwrap().type_ == TT_MINUS {
                operators += if self.current_tok.clone().unwrap().type_ == TT_PLUS { "+" } else { "-" };
                self.next();
                if self.current_tok.is_none() {
                    return Result::Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
                }
                children.push(match self.term() {
                    Ok(n) => n,
                    Err(e) => return Result::Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.clone().last().unwrap().pos_end;
            return Result::Ok(ASTNode::new(ASTNodeType::ArithOp,
                                           children,
                                           pos_start,
                                           pos_end,
                                           Some(operators)));
        }

        Result::Ok(node)
    }

    fn term(&mut self) -> Result<ASTNode, Error> {
        let node = match self.power() {
            Ok(n) => n,
            Err(e) => return Result::Err(e)
        };
        if self.current_tok.is_none() {
            return Result::Ok(node);
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
                    return Result::Err(Error::new(self.previous_end.clone(), self.previous_end.advance(), line!() as u16, String::new()));
                }
                children.push(match self.power() {
                    Ok(n) => n,
                    Err(e) => return Result::Err(e)
                });
                if self.current_tok.is_none() {
                    break;
                }
            }
            let pos_end = children.clone().last().unwrap().pos_end;
            return Result::Ok(ASTNode::new(ASTNodeType::ArithOp,
                                           children,
                                           pos_start,
                                           pos_end,
                                           Some(operators)));
        }

        Result::Ok(node)
    }

    fn power(&mut self) -> Result<ASTNode, Error> {
        self.call()
    }

    fn call(&mut self) -> Result<ASTNode, Error> {
        let node = match self.atom() {
            Ok(n) => n,
            Err(e) => return Result::Err(e)
        };
        Result::Ok(node)
    }

    fn atom(&mut self) -> Result<ASTNode, Error> {
        let tok = self.current_tok.clone().unwrap().clone();
        let pos_start = tok.pos_start.clone();

        let mut out: Option<ASTNode> = None;

        if tok.type_ == TT_INT {
            out = Some(ASTNode::new_v(ASTNodeType::Int, pos_start, tok.pos_end, Some(tok.value)))
        } else if tok.type_ == TT_FLOAT {
            out = Some(ASTNode::new_v(ASTNodeType::Float, pos_start, tok.pos_end, Some(tok.value)))
        } else if tok.type_ == TT_STRING {
            out = Some(ASTNode::new_v(ASTNodeType::String, pos_start, tok.pos_end, Some(tok.value)))
        }

        if out.is_some() {
            self.next();
            Result::Ok(out.unwrap())
        } else {
            Result::Err(Error::new(pos_start.clone(),
                                   pos_start.advance(),
                                   9u16,
                                   String::new()))
        }
    }
}
