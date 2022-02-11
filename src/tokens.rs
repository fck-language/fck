//! Token enum
//!
//! It's just the tokens. tada

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
	Keyword(u8, u8),
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