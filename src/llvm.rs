//! Everything LLVM related
//!
//! This generates LLVM IR code. This file does not contain code to handle compilation to
//! executables, or JIT compilation. They're handled in compiling.rs and interpreter.rs

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_manager_builder::*;
use llvm_sys::{ LLVMBuilder, LLVMIntPredicate, LLVMModule, LLVMOpcode };

use std::ffi::{ CStr, CString };
use std::fmt::Formatter;
use std::os::raw::{ c_char, c_uint, c_ulonglong };
use std::ptr::null_mut;
use std::str;

use crate::nodes::{ ASTNode, ASTNodeType };

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

/// Holder for LLVMModule
///
/// Used so that all the CStrings have the same lifetime as the module
/// Code from [Wilfred/bfc](https://github.com/Wilfred/bfc). Thank you very much
#[derive(Debug)]
pub struct Module {
	pub module: *mut LLVMModule,
	pub strings: Vec<CString>
}

impl Module {
	pub fn new_ptr_i8(&mut self, s: &str) -> *const i8 {
		let out = CString::new(s).unwrap();
		let ptr = out.as_ptr();
		self.strings.push(out);
		ptr as *const i8
	}
}

impl std::fmt::Display for Module {
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

/// Initialise LLVM
///
/// Code from [Wilfred/bfc](https://github.com/Wilfred/bfc). Thank you very much
unsafe fn init() {
	LLVM_InitializeAllTargetInfos();
	LLVM_InitializeAllTargets();
	LLVM_InitializeAllTargetMCs();
	LLVM_InitializeAllAsmParsers();
	LLVM_InitializeAllAsmPrinters();
}

/// Main function of the file. Turns the asts into an LLVM module
pub fn ir_to_module(module_name: &str, asts: Vec<ASTNode>) -> Module {
	let mut module: Module;
	unsafe {
		let llvm_module = LLVMModuleCreateWithName(CString::new(module_name).unwrap().as_bytes_with_nul().as_ptr() as *const c_char);
		LLVMSetTarget(llvm_module, get_default_target_triple().as_ptr() as *const _);
		module = Module { module: llvm_module, strings: vec![] };
	}
	
	// Create the main function that returns an i32 and make a block within that
	let main_fn;
	let mut bb;
    unsafe {
        let main_type = LLVMFunctionType(LLVMInt32Type(), vec![].as_mut_ptr(), 0, LLVM_FALSE);
        main_fn = LLVMAddFunction(module.module, module.new_ptr_i8("main"), main_type);
		bb = LLVMAppendBasicBlock(main_fn, module.new_ptr_i8("body"));
    }
	
	let mut val = int32(0);
	for ast in asts {
		let out = build_ast(&mut module, bb, val, ast);
		bb = out.0;
		val = out.1;
	}
	
	// Add in a ret i32 0 at the end so it doesn't have a meltdown
	unsafe {
		let builder = LLVMCreateBuilder();
        LLVMPositionBuilderAtEnd(builder, bb);
		LLVMBuildRet(builder, int32(0));
	}
	
	module
}

fn int32(val: c_ulonglong) -> LLVMValueRef {
    unsafe { LLVMConstInt(LLVMInt32Type(), val, LLVM_FALSE) }
}

/// Gets the default target string
pub fn get_default_target_triple() -> CString {
    let target_triple;
    unsafe {
        let target_triple_ptr = LLVMGetDefaultTargetTriple();
        target_triple = CStr::from_ptr(target_triple_ptr as *const _).to_owned();
        LLVMDisposeMessage(target_triple_ptr);
    }

    target_triple
}

///
fn build_ast(module: &mut Module, bb: LLVMBasicBlockRef, val: LLVMValueRef, ast: ASTNode) -> (LLVMBasicBlockRef, LLVMValueRef) {
	let mut bb = bb;
	let mut val = val;
	unsafe {
		let out = match ast.node_type {
			// Terminal values
			ASTNodeType::VarAssign(ret, var_type, name) => build_var_assign(module, bb, val, ret, var_type, name, ast.child_nodes),
			ASTNodeType::ArithOp(v) => build_arith_op(module, bb, val, v, ast.child_nodes),
			_ => (bb, val)
		};
		bb = out.0;
		val = out.1;
	}
	(bb, val)
}

unsafe fn build_var_assign(module: &mut Module, bb: LLVMBasicBlockRef, val: LLVMValueRef, ret: bool, var_type: u8, name: String, children: Vec<ASTNode>) -> (LLVMBasicBlockRef, LLVMValueRef) {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	let var = LLVMBuildAlloca(
		builder,
		LLVMInt32Type(),
		module.new_ptr_i8(&*name) as *const c_char
	);
	LLVMBuildStore(
		builder,
		int32(69),
		var
	);
	(bb, val)
}

unsafe fn build_arith_op(module: &mut Module, bb: LLVMBasicBlockRef, val: LLVMValueRef, v: Vec<char>, children: Vec<ASTNode>) -> (LLVMBasicBlockRef, LLVMValueRef) {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	
	(bb, val)
}
