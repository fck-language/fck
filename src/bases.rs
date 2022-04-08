//! All the base structs and one enum that everything else builds upon

use llvm_sys::prelude::{LLVMBuilderRef, LLVMValueRef};
use type_things::prelude::{Module, LLVMMemoryTime, Value};

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

/// Display for position of the form `[ln:{ln:03}, col:{col:03}]`
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ln:{:03}, col:{:03}]", self.ln, self.col)
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

/// Token type enum
///
/// Contains all the possible token types with each type holding the required data
#[derive(PartialEq, Clone, Debug)]
pub enum TokType {
	/// Integer literal
	Int(u64),
	/// Float literal
	Float(f64),
	/// String literal
	String(String),
	/// Plus operator
	Plus,
	/// Minus operator
	Minus,
	/// Modulus operator
	Mod,
	/// Multiply operator
	Mult,
	/// Divide operator
	Div,
	/// Floor divide operator
	FDiv,
	/// Power operator
	Pow,
	/// Left parentheses (
	LParen,
	/// Right parentheses )
	RParen,
	/// Left curly parentheses {
	LParenCurly,
	/// Right curly parentheses }
	RParenCurly,
	/// Left square parentheses [
	LParenSquare,
	/// Right square parentheses ]
	RParenSquare,
	/// At identifier
	At(String),
	/// Boolean negation
	Not,
	/// Colon
	Colon,
	/// Identifier. Holds identifier string and language key
	Identifier(String, String),
	/// Keyword
	Keyword(u8, u16),
	/// Question mark
	QuestionMark,
	/// Dot
	Dot,
	/// Equals operator
	Eq,
	/// Not equals operator
	NE,
	/// Less than operator
	LT,
	/// Greater than operator
	GT,
	/// Less than or equals operator
	LTE,
	/// Greater than or equals operator
	GTE,
	/// Comma
	Comma,
	/// New line
	Newline,
	/// Variable assignment
	Set(bool),
	/// Add value to current variable
	SetPlus(bool),
	/// Subtract value from current variable
	SetMinus(bool),
	/// Modulus current variable
	SetMod(bool),
	/// Multiply variable
	SetMult(bool),
	/// Divide variable
	SetDiv(bool),
	/// Floor divide variable
	SetFDiv(bool),
	/// Power current variable
	SetPow(bool),
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