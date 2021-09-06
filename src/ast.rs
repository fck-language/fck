use crate::tokens::*;
use crate::bases::*;
use crate::err_wrn::*;
use lang::keywords::Keywords;
use crate::nodes::{ASTNode, ASTNodeType};
use lang::get_associated_keywords;

pub struct Lexer {
    split_text: Vec<char>,
    current_pos: Position,
    char_index: usize,
    current_char: char,
    pub(crate) keywords: Keywords<'static>,
    pub(crate) keyword_code: String
}

impl Lexer {
    pub fn new(full_text: String, keywords: Keywords<'static>, keyword_code: String) -> Lexer {
        return Lexer {
            split_text: full_text.chars().collect(),
            current_pos: Position::new(),
            char_index: 0,
            current_char: full_text.chars().nth(0).unwrap(),
            keywords,
            keyword_code
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
                let pos_start = self.current_pos.generate_position();
                if self.current_char == '\n' {
                    if !(self.char_index == self.split_text.len() - 1) {
                        tokens.push(Token::new(TT_NEWLINE, "".into(), pos_start,
                                               self.current_pos.generate_position()));
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
                                                                  self.current_pos.generate_position(),
                                                                  0u16,
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
                                               self.current_pos.generate_position()));
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
                        _ => Result::Err(Error::new())
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
        let pos_start = self.current_pos.generate_position();
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
                          self.current_pos.generate_position());
    }

    fn make_identifier(&mut self) -> Token {
        let pos_start = self.current_pos.generate_position();
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

        return Token::new(tok_type, value, pos_start, self.current_pos.generate_position());
    }

    fn make_loop_identifier(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        if !self.current_char.is_alphanumeric() || self.current_char == '_' {
            return Result::Err(Error::new(pos_start, pos_start.clone().advance(), 0u16, String::new()));
        };
        let mut value = self.current_char.to_string();
        while (self.current_char.is_alphanumeric() || self.current_char == '_') &&
            self.char_index < self.split_text.len() {
            value.push(self.current_char);
            self.advance()
        };
        Result::Ok(Token::new(TT_AT, value, pos_start,
                              self.current_pos.generate_position()))
    }

    fn make_not_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        if self.current_char == '=' {
            return Result::Ok(Token::new(TT_NE, "".into(), pos_start,
                                         self.current_pos.generate_position()));
        } else if self.current_char.is_alphabetic() || "_!".contains(self.current_char) {
            return Result::Ok(Token::new(TT_NOT, "".into(), pos_start,
                                         self.current_pos.generate_position()));
        }
        return Result::Err(Error::new(pos_start, self.current_pos.generate_position(), 0u16, String::new()));
    }

    fn make_equals(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.generate_position();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return Result::Ok(
                Token::new(TT_EQ, "".into(),
                           pos_start,
                           self.current_pos.generate_position()));
        }
        // self.advance();
        return Result::Err(Error::new(pos_start, self.current_pos.generate_position(), 0u16, String::new()));
    }

    fn single_double_token(&mut self, second_char: char, single_type: u8, double_type: u8) -> Result<Token, Error> {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        let mut tok_type = 0;
        if self.current_char == second_char {
            self.advance();
            tok_type = double_type;
        } else {
            tok_type = single_type;
        }
        return Result::Ok(Token::new(tok_type, "".into(), pos_start,
                                     self.current_pos.generate_position()));
    }

    fn make_set(&mut self) -> Result<Token, Error> {
        let pos_start = self.current_pos.generate_position();
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
                        _ => return Result::Err(Error::new(pos_start, self.current_pos.generate_position(), 0u16, String::new()))
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
                        _ => return Result::Err(Error::new(pos_start, self.current_pos.generate_position(), 0u16, String::new()))
                    }
                }
                _ => return Result::Err(Error::new(pos_start, self.current_pos.generate_position(), 0u16, String::new()))
            } - 2;
        }
        tok_type += match self.current_char {
            ':' => 32,
            '>' => 40,
            _ => {
                self.advance();
                return Result::Ok(Token::new(TT_COLON, "".into(), pos_start,
                                             self.current_pos.generate_position()));
            }
        };
        self.advance();
        return Result::Ok(Token::new(tok_type, "".into(), pos_start,
                                     self.current_pos.generate_position()));
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

pub struct ParserResult {
    pub error: Option<Error>,
    pub ast: Vec<ASTNode>,
}

impl ParserResult {
    pub fn new() -> ParserResult {
        return ParserResult { error: None, ast: Vec::new() };
    }
}

pub struct Parser {
    to_process: Vec<Token>,
    intermediate: Vec<Token>,
    processed: Vec<Token>,
    current_tok: Token,
    previous_end: Position
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens;
        tokens.reverse();
        let current_tok = tokens.clone().pop().unwrap();
        return Parser {
            to_process: tokens,
            intermediate: Vec::new(),
            processed: Vec::new(),
            current_tok: current_tok.clone(),
            previous_end: current_tok.pos_start
        };
    }

    // Navigating tokens functions
    fn next(&mut self) -> bool {
        // Goes to the next token and adds the previous one to the intermediate vector
        // Returns true if more tokens are left and false otherwise
        self.previous_end = self.current_tok.pos_end;
        match self.to_process.pop() {
            Some(t) => {
                self.current_tok = t;
                true
            }
            None => { false }
        }
    }
    fn safe_next(&mut self) -> bool {
        self.intermediate.append(&mut Vec::from(vec![self.to_process.remove(0)]));
        self.next()
    }
    fn finalise(&mut self) {
        // Called when the previously processed tokens can safely be moved into the processed
        // vector and the intermediate vector can be cleared
        self.processed.append(&mut self.intermediate);
        self.processed = Vec::new()
    }
    fn put_back(&mut self) {
        todo!()
    }

    // General useful functions
    fn skip_newlines(&mut self) -> bool {
        while self.current_tok.matches(TT_NEWLINE, "".into()) {
            if !self.next() {
                return false;
            }
        }
        true
    }

    // Parsing tokens
    pub fn parse(&mut self) -> Result<Vec<ASTNode>, Error> {
        let mut out: Vec<ASTNode> = Vec::new();

        // Loop through tokens to create an ast
        loop {
            match self.next() {
                true => {}
                false => { break; }
            }

            if !self.skip_newlines() {
                break;
            }

            match self.statement() {
                Ok(AST) => out.push(AST),
                Err(error) => return Result::Err(error)
            }
        }
        return Result::Ok(out);
    }

    fn statement(&mut self) -> Result<ASTNode, Error> {
        let pos_start = self.current_tok.pos_start.clone();
        let mut pos_end = self.current_tok.pos_end.clone();

        // return keyword
        if self.current_tok.matches(TT_KEYWORD, "0.15") {
            if !self.next() || self.current_tok.type_ == TT_NEWLINE {
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
        if self.current_tok.matches(TT_KEYWORD, "0.16") ||
            self.current_tok.matches(TT_KEYWORD, "0.17") {
            let node_type = match &self.current_tok.value as &str {
                "0.16" => ASTNodeType::Continue,
                "0.17" => ASTNodeType::Break,
                _ => { panic!("Why are you here?!") }
            };

            if self.next() {
                // TODO: check for loop identifiers here and process appropriately
                return Result::Err(Error::new(pos_start, self.current_tok.pos_end, 0u16, String::new()));
            }
            if self.current_tok.type_ == TT_NEWLINE {
                return Result::Ok(ASTNode::new(node_type,
                                               Vec::new(),
                                               pos_start,
                                               pos_end,
                                               None));
            }
            if self.current_tok.type_ == TT_AT {}
        }

        self.expr()
    }

    fn expr(&mut self) -> Result<ASTNode, Error> {
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

        let pos_start = self.current_tok.pos_start.clone();
        let mut pos_end = self.current_tok.pos_end.clone();

        // check for new variable assignments
        if self.current_tok.matches_list(1) {
            let var_type = self.current_tok.value.clone().get(2..).unwrap().parse::<u8>().unwrap();
            if !self.next() {
                return Result::Err(Error::new(self.previous_end, self.previous_end.advance().clone(), 0u16, String::new()));
            };
            if self.current_tok.type_ != TT_IDENTIFIER {
                return Result::Err(Error::new(self.current_tok.pos_start, self.current_tok.pos_end, 0u16, String::new()));
            }
            let var_name = self.current_tok.value.clone();
            pos_end = self.current_tok.pos_end.clone();
            let expr: ASTNode;

            if !self.next() {
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
            pos_end = self.current_tok.pos_end.clone();
            match self.current_tok.type_ {
                TT_SET => {
                    if !self.next() {
                        return Result::Err(Error::new(self.previous_end, self.previous_end.advance().clone(), 0u16, String::new()));
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
                    pos_end = self.current_tok.pos_end.clone();
                    if !self.next() || self.current_tok.type_ == TT_NEWLINE {
                        let (node_type, value) = default_values(var_type);
                        expr = ASTNode::new(node_type,
                                            Vec::new(),
                                            Position::new(),
                                            Position::new(),
                                            value);
                        if self.current_tok.type_ == TT_NEWLINE {
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
                _ => return Result::Err(Error::new(self.current_tok.pos_start, self.current_tok.pos_start, 0u16, String::new()))
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
        if self.current_tok.type_ == TT_IDENTIFIER {
            let var_name = self.current_tok.value.clone();
            if !self.next() {
                return Result::Ok(ASTNode::new(ASTNodeType::VarAccess,
                                               Vec::new(),
                                               pos_start,
                                               pos_end,
                                               Some(var_name)))
            }
        }

        Result::Err(Error::new(self.current_tok.pos_start, self.current_tok.pos_end, 0u16, String::new()))
    }
}
