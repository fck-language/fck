//! Main structs required by most otherthings
//!
//! These include the type structs, function call signature structs, and wider reaching module
//! struct that holds the `LLVMModule` for compilation. This is here because of circular referencing
use std::{
	collections::HashMap,
	ffi::{ CStr, CString },
	fmt::Formatter
};

use llvm_sys::{
	LLVMModule,
	prelude::{ LLVMValueRef, LLVMBasicBlockRef },
	core::{ LLVMConstNull, LLVMInt32Type, LLVMPrintModuleToString, LLVMDisposeMessage }
};

/// Generic type struct
///
/// This is the generic struct that all the other types are derived from. It's probably fairly
/// inefficient because I'm having to make this up as I go along and haven't read up on it yet but
/// it works so leave me alone
pub struct fckType {
	/// Names of the type for different languages. The keys are the language codes and the values
	/// are the names in that language. This will probably change in a bit
	names: HashMap<String, String>,
	/// Functions that the type implements. The keys are the function names, which are converted by
	/// the parser into the original identifiers, and the values are the function
	functions: HashMap<String, FuncCallSig>
}

/// Function holder for the type struct
///
/// This holds the arguments, return type ID, and function used by the compiler to run the function
#[derive(Hash)]
pub struct FuncCallSig {
	/// Function arguments (ordered)
	args: Vec<FuncArg>,
	/// Return type ID
	ret: u8,
	/// Function used th compile the function into LLVM IR
	body: fn () -> u8
}

/// Function argument
///
/// Holds the argument name and type ID
#[derive(Hash)]
pub struct FuncArg {
	/// Argument name
	name: String,
	/// Argument type ID
	type_: u8
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
	pub current_fn: LLVMValueRef
}

impl Module {
	/// Initialise a new Module
	pub unsafe fn new(module: *mut LLVMModule) -> Self {
		let blank = CString::new("").unwrap();
		Module { module, blank, strings: vec![], current_fn: LLVMConstNull(LLVMInt32Type()) }
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

impl std::fmt::Display for Module {
	/// Returns the LLVM IR code
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
