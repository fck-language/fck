use crate::tokens::TT_KEYWORD;
use crate::types::*;

use std::collections::HashMap;
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    ln: usize,
    pub col: usize
}

impl Position {
    pub fn new() -> Position {
        return Position{ln: 0, col: 0}
    }
    pub fn advance(mut self) -> Self {
        self.col += 1;
        self
    }
    pub fn advance_ln(&mut self) {
        self.ln += 1;
        self.col = 0
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ln:{:02}, col:{:02}]", self.ln, self.col)
    }
}

#[derive(Clone)]
pub struct Token {
    pub type_: u8,
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position
}

impl Token {
    pub fn new(type_: u8, value: String, pos_start: Position, pos_end: Position) -> Token {
        return Token{type_, value, pos_start, pos_end};
    }
    pub fn matches(&self, type_: u8, value: &str) -> bool {
        return self.type_ == type_ && self.value == value;
    }
    pub fn matches_list(&self, list: u8) -> bool {
        self.type_ == TT_KEYWORD && self.value.clone().get(0..1).unwrap() == format!("{}", list)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type: {:<3} (Value: {})\n\tpos_start: {}\n\tpos_end  : {}",
               self.type_, self.value, self.pos_start, self.pos_end)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "T:{:<3} V:{} ps:{} pe:{}", self.type_, self.value, self.pos_start, self.pos_end)
    }
}


pub struct Context<'a> {
    display_name: String,
    full_text: String,
    parent: Option<&'a Context<'a>>
}

impl Context<'_> {
    pub fn new<'a>(display_name: String, full_text: String, parent: Option<&'a Context>) -> Context<'a> {
        Context{display_name, full_text, parent}
    }
}

pub struct SymbolTable {
    display_name: String,
    parent: Box<Option<SymbolTable>>,
    variables: HashMap<String, Type>,
    names_loops: Vec<String>,
}
