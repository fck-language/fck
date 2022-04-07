//! Base file with all the "building blocks" of everything else in

use crate::tokens::TokType;
// use crate::types::*;

use std::collections::HashMap;

/// Position container. Is the basis for positions of tokens and nodes.
///
/// In documentation and comments, this struct will be represented as `({ln}{col})`
#[derive(Copy, Clone, Debug)]
pub struct Position {
    /// Line number
    ln: usize,
    /// Column number
    pub col: usize
}

impl Position {
    /// Creates a new position at (0,0)
    pub fn new() -> Position {
        return Position{ln: 0, col: 0}
    }
    /// Advances and returns the position by adding one to `self.col`
    pub fn advance(mut self) -> Self {
        self.col += 1;
        self
    }
    /// Adds one to `self.ln` and sets `self.col` to 0
    pub fn advance_ln(&mut self) {
        self.ln += 1;
        self.col = 0
    }
}

/// Display for position of the form `[ln:{ln:02}, col:{col:02}]`
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ln:{:02}, col:{:02}]", self.ln, self.col)
    }
}

/// Token struct. Used by the lexer and parser
#[derive(Clone)]
pub struct Token {
    /// The token type from `tokens.rs`
    pub type_: TokType,
    /// Starting position
    pub pos_start: Position,
    /// Ending position. This is really one after the ending position so `test` would start at (0,0)
    /// and end at (0,4)
    pub pos_end: Position
}

/// Allow us to check if a token is of a specific type
impl PartialEq<TokType> for Token {
    fn eq(&self, other: &TokType) -> bool {
        &self.type_ == other
    }
}

/// Allow us to check if a token is a keyword in a specific list
impl PartialEq<u8> for Token {
    fn eq(&self, other: &u8) -> bool {
        if let TokType::Keyword(v, _) = self.type_ {
            &v == other
        } else {
            false
        }
    }
}

impl Token {
    /// Generates a new token from a given token type, value, and starting and ending positions
    pub fn new(type_: TokType, pos_start: Position, pos_end: Position) -> Token {
        return Token{type_, pos_start, pos_end};
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type: {:<3?} \n\tpos_start: {}\n\tpos_end  : {}",
               self.type_, self.pos_start, self.pos_end)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "T:{:<3?}  ps:{} pe:{}", self.type_, self.pos_start, self.pos_end)
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
    // variables: HashMap<String, Type>,
    names_loops: Vec<String>,
}