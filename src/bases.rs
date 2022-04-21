//! All the base structs and one enum that everything else builds upon

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Formatter;
use std::os::raw::c_ulonglong;
use llvm_sys::{
	prelude::{LLVMBuilderRef, LLVMValueRef, LLVMBool},
	core::{LLVMBuildAlloca, LLVMConstInt, LLVMInt64Type}
};
use llvm_sys::core::LLVMBuildStore;
use type_things::prelude::{Module, LLVMMemoryTime, Value};
use colored::*;
use colored::Styles::Bold;
use crate::nodes::{ASTNode, ASTNodeType};

/// True constant of type `LLVMBool`
pub const LLVM_TRUE: LLVMBool = 1;

/// Position container. Is the basis for positions of tokens and nodes.
///
/// In documentation and comments, this struct will be represented as `({ln}{col})`
#[derive(Copy, Clone, Debug)]
pub struct Position {
    /// Line number
    pub ln: usize,
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
        write!(f, "Type: {:<3?} \n\t{pad}pos_start: {}\n\t{pad}pos_end  : {}",
               self.type_, self.pos_start, self.pos_end, pad=String::from(' ').repeat(f.width().unwrap_or(0)))
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

/// Parser symbol table to hold in-scope variables
///
/// This holds all the variables defined exclusively in the same scope. This scope can optionally
/// have a name which is used in the cases of branching statements
#[derive(Clone)]
pub struct SymbolTable {
    /// Variables in the current scope
    variables: Vec<String>,
	/// Variable aliases. The values are the index of the
	aliases: HashMap<String, usize>,
    /// Checking map. This holds all the same keys as the `variables` map, with the associated value
    /// only being `true` when the variable has been used once. Each variable that is unused is
    /// removed from the `variables` map to save on memory
    checker: Vec<bool>,
	/// Scope index. This tells the symbol table which symbol table to look at for variables
	scope_index: Vec<usize>,
    /// Scope name. Used by branching statements
    name: Option<String>
}

impl SymbolTable {
	/// Generate a new symbol table. This is only used to generate a root symbol table
	pub fn new() -> Self {
		SymbolTable {
			variables: vec![],
			aliases: HashMap::new(),
			checker: vec![],
			scope_index: vec![],
			name: None
		}
	}
	
	/// Create a new child symbol table
	pub fn new_child(&self, new_index: usize, name: Option<String>) -> Self {
		let mut scope_index = self.scope_index.clone();
		scope_index.push(new_index);
		SymbolTable {
			variables: vec![],
			aliases: HashMap::new(),
			checker: vec![],
			scope_index,
			name
		}
	}
	
	/// Pushes a new variable to the symbol table. The name **must** start with a language code
	pub fn push(&mut self, name: String) {
		self.variables.push(name);
		self.checker.push(false);
	}
}

impl std::fmt::Debug for SymbolTable {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let pad = String::from(' ').repeat(f.width().unwrap_or(0));
		let vars = (0..self.variables.len()).map(|i| {
			let mut temp = self.variables.get(i).unwrap().clone();
			if *self.checker.get(i).unwrap() { temp } else { format!("{}(unused)", temp) }
		}).collect::<Vec<String>>();
		write!(f, "Variables : {}\n{pad}Aliases   : [{}]\n{pad}SI        : {}\n{pad}Name      : {}",
			vars.join(", "),
			if self.aliases.is_empty() {
				String::from("None")
			} else {
				self.aliases.iter().map(
					|(key, ptr)| format!("{} -> {}({})", key, self.variables.get(ptr.clone()).unwrap(), ptr)
				).collect::<Vec<String>>().join(",")
			},
			if self.scope_index.is_empty() {
				String::from("Root")
			} else {
				self.scope_index.iter().map(|i| format!("{:>03}", i)).collect::<Vec<String>>().join(" -> ")
			},
			self.name.as_ref().unwrap_or(&String::from("No name")),
			pad = pad
		)
	}
}

/// Compiler symbol table
///
/// This is the symbol table used by the compiler
pub struct CompSymbolTable {
	/// Variable values
	values: Vec<Value>,
	/// Scope name
	name: Option<String>
}

impl CompSymbolTable {
	/// Push a new value into the symbol table
	///
	/// This takes a value and allocates the space for the value. Stores the value in the allocated
	/// space, pushes the value to the vector, and then returns a reference to the value in the
	/// symbol table vector
	pub unsafe fn push(&mut self, value: Value, builder: LLVMBuilderRef, module: &mut Module) -> Result<&Value, ()> {
		let allocate_type;
		if let Some(t) = module.types.get(value.type_ as usize) {
			allocate_type = (t.llvm_type)()
		} else {
			return Err(())
		}
		let var = LLVMBuildAlloca(builder, allocate_type, module.blank.as_ptr());
		let out = LLVMBuildStore(builder, value.value, var);
		self.values.push(Value {
			value: out,
			type_: value.type_
		});
		Ok(self.values.last().unwrap())
	}
}

/// Allow us to cast a `SymbolTable` into a `CompSymbolTable`
impl std::convert::Into<CompSymbolTable> for SymbolTable {
	fn into(self) -> CompSymbolTable {
		CompSymbolTable {
			values: Vec::with_capacity(self.variables.len()),
			name: self.name
		}
	}
}
