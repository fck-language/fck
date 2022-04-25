//! Main structs required by most otherthings
//!
//! These include the type structs, function call signature structs, and wider reaching module
//! struct that holds the `LLVMModule` for compilation. This is here because of circular referencing
use std::{
	ffi::{ CStr, CString },
	fmt::{ Formatter, Display, Debug },
	hash::{ Hash, Hasher },
	os::raw::c_char
};

use llvm_sys::{
	LLVMModule,
	prelude::{ LLVMValueRef, LLVMBasicBlockRef, LLVMBuilderRef, LLVMTypeRef },
	core::{
		LLVMConstNull, LLVMInt32Type, LLVMPrintModuleToString, LLVMDisposeMessage, LLVMBuildAlloca
	},
	LLVMValue
};
use phf::Map;

use crate::primitives::{INT, NULL_TYPE};
use crate::symbol_tables::CompSymbolTable;

/// LLVM memory allocation trait
///
/// This is implemented by anything that requires memory allocation (and eventually deallocation)
pub trait LLVMMemoryTime<T> {
	/// Allocate the required memory for the struct. This returns the value ref pointing to the
	/// allocated space
    unsafe fn allocate(&self, builder: LLVMBuilderRef, module: &mut Module) -> T;
	/// Deallocate the memory the struct previously held
    unsafe fn deallocate(&self, builder: LLVMBuilderRef, module: &mut Module);
}

/// fck Type
///
/// This struct contains the functions and names for a type. It does not contain the type ID or a
/// value. The type ID comes from teh index of a type in the list of all used types, and the value
/// of a type is contained within a separate struct
pub struct Type {
	/// Names of the type for different languages. The keys are the language codes and the values
	/// are the names in that language. This will probably change in a bit
	pub names: Map<&'static str, &'static str>,
	/// Operations that the type implement
	/// Each function requires the `Module`, and current `LLVMBasicBlockRef` along with:
	/// 1. The value calling the function (LHS)
	/// 2. The second value (RHS)
	///
	/// Most of the time if these are flipped it won't matter, but it's always nice not to have to
	/// "find a bug" only to realise the arguments were the wrong way around
	pub ops: Map<&'static str, Map<u16, unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)>>,
	/// Functions that the type implements. The keys are the function names, which are converted by
	/// the parser into the original identifiers, and the values are the function
	pub functions: Map<&'static str, FuncCallSig>,
	/// LLVM type. We use this to allocate space in the `LLVMMemoryTime` trait
	pub llvm_type: unsafe fn () -> LLVMTypeRef
}

impl Debug for Type {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{:?}, {}{}", '{', self.names,
						  self.ops.entries().fold(String::new(), |acc, (k, v)| {
							  format!("{}, ({} => {:?})", acc, k, v.keys().fold(String::new(), |sub_acc, sk| format!("{}, {}", acc, sk)))
						  }),
						  '}')
	}
}

/// Value struct for holding a value
///
/// This struct holds a value and type ID for that value
#[derive(Copy, Clone, Debug)]
pub struct Value {
	pub value: LLVMValueRef,
	pub type_: u16
}

impl Value {
	pub fn new(value: LLVMValueRef, type_: u16) -> Self {
		Value{ value, type_ }
	}
}

/// Function holder for the type struct
///
/// This holds the arguments, return type ID, and function used by the compiler to run the function
pub struct FuncCallSig {
	/// Function arguments (ordered)
	pub(crate) args: Vec<FuncArg>,
	/// Return type ID
	pub(crate) ret: u16,
	/// Function used to compile the function into LLVM IR
	///
	/// The two `LLVMValueRef`s are for the previous value (i.e. a returned one) and the value of
	/// the type instance respectively
	pub(crate) body: unsafe fn (&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)
}

impl Hash for FuncCallSig {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.args.iter().map(|x| state.write_u16(x.type_.clone()));
		state.write_u16(self.ret.clone());
		state.finish();
	}
}

impl Debug for FuncCallSig {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{:?}, {}{}", '{', self.args, self.ret, '}')
	}
}

/// Function argument
///
/// Holds the argument name and type ID
#[derive(Hash, Debug)]
pub struct FuncArg {
	/// Argument name
	pub(crate) name: &'static str,
	/// Argument type ID
	pub(crate) type_: u16
}

/// Holder for LLVMModule
///
/// Modified from [Wilfred/bfc](https://github.com/Wilfred/bfc). Thank you very much
/// Holds the current function, LLVMModule, named blocks (as CString) to ensure concurrent
/// lifetimes, as well as a blank CString (pub) to be used as the default block name
#[derive(Debug)]
pub struct Module {
	/// LLVMModule that things get build to
	pub module: *mut LLVMModule,
	/// Blank CString used as the default block name
	pub blank: CString,
	/// Vector holding all the named block names
	strings: Vec<CString>,
	/// Current function to make sure that any new blocks are added to the right function
	pub current_fn: LLVMValueRef,
	/// All the types that are currently being used. This is initialised to having just the
	/// primitives and null type
	pub types: Vec<Type>,
	/// Holds the symbol tables in scope
	current_scope: Vec<CompSymbolTable>,
	/// Holds the remaining symbol tables. These each have the correct capacity, but are empty
	remaining_symbol_tables: Vec<CompSymbolTable>
}

impl Module {
	/// Initialise a new Module
	pub unsafe fn new(module: *mut LLVMModule, mut remaining_symbol_tables: Vec<CompSymbolTable>) -> Self {
		let blank = CString::new("").unwrap();
		Module {
			module, blank, strings: vec![],
			current_fn: LLVMConstNull(LLVMInt32Type()), types: vec![NULL_TYPE, INT],
			current_scope: vec![remaining_symbol_tables.pop().unwrap()],
			remaining_symbol_tables
		}
	}
	/// Make a new CString and return it as a `*const i8`
	///
	/// This is a short-hand function for quality of life improvements, as well as required to
	/// ensure the CString is added to the module, ensuring the same lifetime as the module
	pub fn new_ptr_i8(&mut self, s: &str) -> *const i8 {
		let out = CString::new(s).unwrap();
		let ptr = out.as_ptr();
		self.strings.push(out);
		ptr as *const i8
	}
}

impl Display for Module {
	/// Returns the LLVM IR code for the module (specifically `self.module`)
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let module_string;
		unsafe {
			let llvm_ir_ptr = LLVMPrintModuleToString(self.module);
			let llvm_ir = CStr::from_ptr(llvm_ir_ptr as *const _);
			module_string = CString::new(llvm_ir.to_bytes()).unwrap();
			LLVMDisposeMessage(llvm_ir_ptr);
		}
		write!(f, "{}", String::from_utf8_lossy(module_string.as_bytes()).to_string())
	}
}
