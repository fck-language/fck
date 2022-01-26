//! Base file with all the "building blocks" of everything else in

use std::collections::HashMap;
use llvm_sys::prelude::LLVMValueRef;

use crate::tokens::TT_KEYWORD;

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

impl std::fmt::Display for Position {
    /// Writes the line and column in that order with a width of two characters
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ln:{:02}, col:{:02}]", self.ln, self.col)
    }
}

/// Token struct. Used by the lexer and parser
#[derive(Clone)]
pub struct Token {
    /// The token type from `tokens.rs`
    pub type_: u8,
    /// Value stored in a string
    pub value: String,
    /// Starting position
    pub pos_start: Position,
    /// Ending position. This is really one after the ending position so `test` would start at (0,0)
    /// and end at (0,4)
    pub pos_end: Position
}

impl Token {
    /// Generates a new token from a given token type, value, and starting and ending positions
    pub fn new(type_: u8, value: String, pos_start: Position, pos_end: Position) -> Token {
        return Token{type_, value, pos_start, pos_end};
    }
    /// Checks is a token matches a specific type and value
    pub fn matches(&self, type_: u8, value: &str) -> bool {
        return self.type_ == type_ && self.value == value;
    }
    /// Checks if a token is a keyword in a specific keyword list
    pub fn matches_list(&self, list: u8) -> bool {
        self.type_ == TT_KEYWORD && self.value.clone().get(0..1).unwrap() == format!("{}", list)
    }
}

impl std::fmt::Display for Token {
    /// Writes the type, value, and starting and ending positions
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type: {:<3} (Value: {})\n\tpos_start: {}\n\tpos_end  : {}",
               self.type_, self.value, self.pos_start, self.pos_end)
    }
}

impl std::fmt::Debug for Token {
    /// Writes the type, value, and starting and ending positions with shortened prefixes
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "T:{:<3} V:{} ps:{} pe:{}", self.type_, self.value, self.pos_start, self.pos_end)
    }
}


/// Contained for all the local variable names
///
/// Contains the variable names, aliases for variables, and an optional name for the lifetime
pub struct Lifetime {
    /// Variables stored as the original language keyed name for the key and variable as the value
    variables: HashMap<String, Variable>,
    /// Aliases for the current local variables
    aliases: HashMap<String, String>,
    /// Optional lifetime name. Used when loops have identifier allowing you to break specific loops
    name: Option<String>
}

/// Container for variables
///
/// Contains the variable type ID and LLVM value
pub struct Variable {
    /// Type ID unique to the variable type
    var_type: u16,
    /// LLVM value ref
    value: LLVMValueRef
}
