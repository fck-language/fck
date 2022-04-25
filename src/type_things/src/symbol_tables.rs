//! Symbol table structs
//!
//! This contains both the `SymbolTable` and `CompSymbolTable` for the ast and compiler respectively

use std::{
	collections::HashMap,
	convert::From,
	fmt::{ Display, Debug, Formatter },
	os::raw::c_ulonglong
};
use llvm_sys::{
	prelude::{ LLVMBuilderRef },
	core::{ LLVMBuildStore, LLVMBuildAlloca }
};

use crate::prelude::{ Value, Module };

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
	pub scope_index: Vec<usize>,
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
			scope_index: vec![0],
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
	
	pub fn find(&self, name: &String) -> Option<usize> {
		self.variables.iter().position(|f| f == name)
	}
	
	pub fn found(&mut self, index: usize) {
		self.checker[index] = true;
	}
}

impl Debug for SymbolTable {
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
#[derive(Debug)]
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
impl From<&SymbolTable> for CompSymbolTable {
	fn from(t: &SymbolTable) -> Self {
		CompSymbolTable {
			values: Vec::with_capacity(t.variables.len()),
			name: t.name.clone()
		}
	}
}
