use crate::tokens::*;
use crate::bases::*;
use crate::ErrWrn::*;
use lang::keywords::Keywords;
use crate::nodes::{ASTNode, ASTNodeType};
use std::slice::Iter;

pub struct Lexer<'a> {
    split_text: Vec<char>,
    current_pos: Position,
    char_index: usize,
    current_char: char,
    keywords: Keywords<'a>
}

impl Lexer<'_> {
    pub fn new(full_text: String, keywords: Keywords) -> Lexer {
        return Lexer{split_text: full_text.chars().collect(), current_pos: Position::new(),
            char_index: 0, current_char: full_text.chars().nth(0).unwrap(),
            keywords}
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
    pub fn make_tokens(&mut self) -> Vec<Token> {
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
                        '@' => TT_AT,
                        '.' => TT_DOT,
                        _ => 0
                    };
                    if tok_type > 0 {
                        self.advance();
                        tokens.push(Token::new(tok_type, "".into(), pos_start,
                                               self.current_pos.generate_position()));
                        continue;
                    }

                    let (res, tok, err) = match self.current_char {
                        '!' => self.make_not_equals(),
                        '<' => self.single_double_token('=', TT_LT, TT_LTE),
                        '>' => self.single_double_token('=', TT_GT, TT_GTE),
                        '*' => self.single_double_token('*', TT_MULT, TT_POW),
                        '/' => self.single_double_token('/', TT_DIV, TT_FDIV),
                        ':' => self.make_set(),
                        '=' => self.make_equals(),
                        _ => (ErrorOrNot::Err, Token::blank(), Error{})
                    };

                    match res {
                        ErrorOrNot::Err => {panic!("A error was found! uh oh...");}
                        ErrorOrNot::NotErr => {tokens.push(tok)}
                    }
                }
            }
        }
        return tokens
    }

    // Character parsing functions
    fn make_number(&mut self) -> Token {
        let mut has_dot = false;
        let pos_start = self.current_pos.generate_position();
        let mut value = String::new();

        while self.current_char.is_numeric() || self.current_char == '.' {
            if self.current_char == '.' {
                if has_dot {
                    break
                }
                has_dot = true;
                value.push('.')
            } else {
                value.push(self.current_char)
            }
            self.advance()
        }

        return Token::new(has_dot as u8, value, pos_start,
                                 self.current_pos.generate_position())
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

        let tok_type = match self.keywords.contains(&keyword.clone() as &str) {
            true => TT_KEYWORD,
            false => TT_IDENTIFIER
        };

        return Token::new(tok_type, keyword, pos_start, self.current_pos.generate_position())
    }

    fn make_not_equals(&mut self) -> (ErrorOrNot, Token, Error) {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        if self.current_char == '=' {
            return (ErrorOrNot::NotErr, Token::new(TT_NE, "".into(), pos_start,
                                                   self.current_pos.generate_position()),
                    Error{})
        } else if self.current_char.is_alphabetic() || "_!".contains(self.current_char) {
            return (ErrorOrNot::NotErr, Token::new(TT_NOT, "".into(), pos_start,
                                                   self.current_pos.generate_position()),
                    Error{})
        }
        return (ErrorOrNot::Err, Token::blank(), Error{});
    }

    fn make_equals(&mut self) -> (ErrorOrNot, Token, Error) {
        let pos_start = self.current_pos.generate_position();
        self.advance();

        if self.current_char == '=' {
            self.advance();
            return (ErrorOrNot::NotErr, Token::new(TT_EQ, "".into(), pos_start, self.current_pos.generate_position()), Error{});
        }
        self.advance();
        return (ErrorOrNot::Err, Token::blank(), Error{});
    }

    fn single_double_token(&mut self, second_char: char, single_type: u8, double_type: u8) -> (ErrorOrNot, Token, Error) {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        let mut tok_type = 0;
        if self.current_char == second_char {
            self.advance();
            tok_type = double_type;
        } else {
            tok_type = single_type;
        }
        return (ErrorOrNot::NotErr, Token::new(tok_type, "".into(), pos_start, self.current_pos.generate_position()), Error{})
    }

    fn make_set(&mut self) -> (ErrorOrNot, Token, Error) {
        let pos_start = self.current_pos.generate_position();
        self.advance();
        let mut tok_type = 0u8;
        if !":>".contains(self.current_char) {
            tok_type += match self.current_char {
                '+' => {self.advance();TT_PLUS},
                '-' => {self.advance();TT_MINUS},
                '%' => {self.advance();TT_MOD},
                '*' => {self.advance();
                match self.current_char {
                    '*' => {self.advance(); TT_POW}
                    ':' => TT_MULT,
                    '>' => TT_MULT,
                    _ => return (ErrorOrNot::Err, Token::blank(), Error{})
                }},
                '/' => {self.advance();
                match self.current_char {
                    '/' => {self.advance(); TT_FDIV}
                    ':' => TT_DIV,
                    '>' => TT_DIV,
                    _ => return (ErrorOrNot::Err, Token::blank(), Error{})
                }},
                _ => return (ErrorOrNot::Err, Token::blank(), Error{})
            } - 2;
        }
        tok_type += match self.current_char {
            ':' => 32,
            '>' => 40,
            _ => {self.advance();
                return (ErrorOrNot::NotErr,
                        Token::new(TT_COLON, "".into(),pos_start,
                                   self.current_pos.generate_position()), Error{})}
        };
        self.advance();
        return (ErrorOrNot::NotErr, Token::new(tok_type, "".into(), pos_start, self.current_pos.generate_position()), Error{});
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

struct ParserResult {
    error: Option<Error>,
    ast: Vec<ASTNode>
}

impl ParserResult {
    pub fn new() -> ParserResult {
        return ParserResult{error: None, ast: Vec::new()}
    }
}

struct ParserFunctionReturn {
    error: Option<Error>,
    node: Option<ASTNode>,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize
}

impl ParserFunctionReturn {
    pub fn new() -> ParserFunctionReturn {
        return ParserFunctionReturn{error: None, node: None,
            last_registered_advance_count: 0, advance_count: 0, to_reverse_count: 0 }
    }
}

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser{tokens}
    }

    pub fn parse(&mut self) -> ParserResult {
        // Setup
        let mut iterator = self.tokens.iter();
        let mut current_tok = iterator.next().unwrap();
        let pos_start = &current_tok.pos_start.clone();
        let mut res = ParserResult::new();
        let mut out = ParserResult::new();
        let mut token_index = 0u128;

        // Loop through tokens to create an ast
        loop {
            while current_tok.matches(TT_NEWLINE, "".into()) {
                token_index += 1;
                current_tok = iterator.next().unwrap();
            }

            let res = self.statement(*iterator, current_tok);
            match res.error {
                Some(Error) => {},
                None => out.ast.push(res.node.unwrap())
            }

            current_tok = match iterator.next() {
                Some(tok) => tok,
                None => break
            };
        }
        // let node1 = ASTNode::new(ASTNodeType::Bool, Vec::new(), Position::new(), Position::new(), None);
        // out.push(ASTNode::new(ASTNodeType::IntNode, Vec::from([node1]), Position::new(), Position::new(), None));
        return out;
    }

    fn statement(&mut self, mut iterator: Iter<Token>, current_tok: &Token) -> ParserFunctionReturn {
        let mut out = ParserFunctionReturn::new();
        let mut current_token = current_tok;
        return out;
    }
}