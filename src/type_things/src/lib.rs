mod checker;

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

pub struct fckType {
	names: HashMap<String, String>,
	functions: HashMap<String, FuncCallSig>
}

#[derive(Hash)]
pub struct FuncCallSig {
	args: Vec<FuncArg>,
	ret: u8,
	body: fn () -> u8
}

#[derive(Hash)]
pub struct FuncArg {
	name: String,
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
