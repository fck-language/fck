use crate::tokens::*;
use crate::bases::*;
use crate::ErrWrn::*;
use lang::keywords::Keywords;
use crate::nodes::{ASTNode, ASTNodeType};

pub struct Lexer<'a> {
    split_text: Vec<char>,
    current_pos: Position,
    char_index: usize,
    current_char: char,
    pub(crate) keywords: Keywords<'a>,
}

impl Lexer<'_> {
    pub fn new(full_text: String, keywords: Keywords) -> Lexer {
        return Lexer {
            split_text: full_text.chars().collect(),
            current_pos: Position::new(),
            char_index: 0,
            current_char: full_text.chars().nth(0).unwrap(),
            keywords,
        };
    }

    fn advance(&mut self) {
        self.current_pos.advance();
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
                    self.skip_comment();
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
            return Result::Err(Error::new());
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
        return Result::Err(Error::new());
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
        self.advance();
        return Result::Err(Error::new());
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
                        _ => return Result::Err(Error::new())
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
                        _ => return Result::Err(Error::new())
                    }
                }
                _ => return Result::Err(Error::new())
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
        self.advance();

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
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let mut tokens = tokens;
        tokens.reverse();
        let current_tok = tokens.clone().pop().unwrap();
        return Parser{to_process: tokens, intermediate: Vec::new(), processed: Vec::new(),
            current_tok
        }
    }

    fn next(&mut self) -> bool {
        /// Goes to the next token and adds the previous one to the intermediate vector
        /// Returns true if more tokens are left and false otherwise
        self.intermediate.append(&mut Vec::from(vec![self.to_process.remove(0)]));
        match self.to_process.pop() {
            Some(T) => {self.current_tok = T; true}
            None => {false}
        }
    }
    fn finalise(&mut self) {
        /// Called when the previously processed tokens can safely be moved into the processed
        /// vector and the intermediate vector can be cleared
        self.processed.append(&mut self.intermediate);
        self.processed = Vec::new()
    }
    pub fn put_back(&mut self) {
        todo!()
    }

    pub fn parse(&mut self) -> ParserResult {
        // Setup
        let pos_start = self.current_tok.pos_start.clone();
        let mut out = ParserResult::new();

        // Loop through tokens to create an ast
        loop {
            while self.current_tok.matches(TT_NEWLINE, "".into()) {
                match self.next() {
                    true => {}
                    false => return out
                };
                self.finalise()
            }

            let res = self.statement();
            match res.error {
                Some(Error) => {},
                None => out.ast.push(res.node.unwrap())
            }

            match self.next() {
                true => {},
                false => {break}
            }
            self.finalise()
        }
        // let node1 = ASTNode::new(ASTNodeType::Bool, Vec::new(), Position::new(), Position::new(), None);
        // out.push(ASTNode::new(ASTNodeType::IntNode, Vec::from([node1]), Position::new(), Position::new(), None));
        return out;
    }

    fn statement(&mut self) -> ParserFunctionReturn {
        let mut out = ParserFunctionReturn::new();
        return out;
    }
}
