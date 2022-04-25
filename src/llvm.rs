//! Everything LLVM related
//!
//! This generates LLVM IR code. This file does not contain code to handle compilation to
//! executables, or JIT compilation. They're handled in compiling.rs and interpreter.rs

use std::{
	ffi::{ CStr, CString },
	os::raw::{ c_char, c_ulonglong },
	ptr::null_mut,
	str
};

use llvm_sys::{
	LLVMIntPredicate, LLVMModule,
	core::*,
	prelude::*,
	target::*,
	target_machine::*
};

use type_things::{
	prelude::{ Value, Type, Module },
	primitives::null_value,
	symbol_tables::CompSymbolTable
};
use crate::nodes::{ ASTNode, ASTNodeType };

/// False constant of type `LLVMBool`
const LLVM_FALSE: LLVMBool = 0;
/// True constant of type `LLVMBool`
const LLVM_TRUE: LLVMBool = 1;

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
pub fn ir_to_module(module_name: &str, asts: Vec<ASTNode>, symbol_tables: Vec<CompSymbolTable>) -> Module {
	let mut module: Module;
	unsafe {
		init();
		let llvm_module = LLVMModuleCreateWithName(CString::new(module_name).unwrap().as_bytes_with_nul().as_ptr() as *const c_char);
		LLVMSetTarget(llvm_module, get_default_target_triple().as_ptr() as *const _);
		module = Module::new(llvm_module, symbol_tables);
	}
	
	// Create the main function that returns an i32 and make a block within that
	let main_fn;
	let mut bb;
	let mut val = null_value();
	unsafe {
		let main_type = LLVMFunctionType(LLVMInt32Type(), vec![].as_mut_ptr(), 0, LLVM_FALSE);
		main_fn = LLVMAddFunction(module.module, module.new_ptr_i8("main"), main_type);
		bb = LLVMAppendBasicBlock(main_fn, module.blank.as_ptr());
	}
	module.current_fn = main_fn;
	
	for ast in asts {
		let out = match build_ast(&mut module, bb, val, ast) {
			Ok((bb_, val_)) => {
				bb = bb_;
				val = val_;
			}
			Err(e) => println!("{}", e)
		};
	}
	
	// Add in a ret i32 0 at the end so it doesn't have a meltdown
	unsafe {
		let builder = LLVMCreateBuilder();
        LLVMPositionBuilderAtEnd(builder, bb);
		LLVMBuildRet(builder, LLVMConstInt(LLVMInt32Type(), 0, LLVM_FALSE));
	}
	
	module
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

/// Builds the ASTs to the module
fn build_ast(module: &mut Module, mut bb: LLVMBasicBlockRef, mut val: Value, ast: ASTNode) -> Result<(LLVMBasicBlockRef, Value), String> {
	unsafe {
		let out = match match ast.node_type {
			// Terminal values
			ASTNodeType::VarAssign(var_type, name) => build_var_assign(module, bb, val, var_type, name, ast.child_nodes.get(0).unwrap().clone()),
			ASTNodeType::ArithOp(v) => build_arith_op(module, bb, val, v, ast.child_nodes),
			ASTNodeType::Int(v) => Ok((bb, Value::new(LLVMConstInt(LLVMInt64Type(), v as c_ulonglong, LLVM_TRUE), 1))),
			ASTNodeType::CompOp(v) => build_comp_op(module, bb, val, v, ast.child_nodes),
			ASTNodeType::If(_) => build_if(module, bb, val, ast.child_nodes),
			_ => Ok((bb, val))
		} {
			Ok((bb_, val_)) => {
				bb = bb_;
				val = val_;
			}
			Err(e) => return Err(e)
		};
	}
	Ok((bb, val))
}

/// Builds a `ASTNodeType::VarAssign` AST node to the module
unsafe fn build_var_assign(module: &mut Module, mut bb: LLVMBasicBlockRef, mut val: Value, _var_type: u16, name: String, value: ASTNode) -> Result<(LLVMBasicBlockRef, Value), String> {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	let var = LLVMBuildAlloca(
		builder,
		LLVMInt32Type(),
		module.new_ptr_i8(&*name) as *const c_char
	);
	let child_out = match build_ast(module, bb, null_value(), value) {
		Ok(t) => t,
		Err(e) => return Err(e)
	};
	bb = child_out.0;
	LLVMBuildStore(
		builder,
		child_out.1.value,
		var
	);
	val = child_out.1.clone();
	Ok((bb, val))
}

/// Builds a `ASTNodeType::ArithOp` AST node to the module
unsafe fn build_arith_op(module: &mut Module, mut bb: LLVMBasicBlockRef, mut val: Value, v: Vec<char>, children: Vec<ASTNode>) -> Result<(LLVMBasicBlockRef, Value), String> {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	let mut children = children.iter();
	let build_out = match build_ast(module, bb, val, children.next().unwrap().clone()) {
		Ok(t) => t,
		Err(e) => return Err(e)
	};
	bb = build_out.0;
	val = build_out.1;
	for c in v {
		let build_out = match build_ast(module, bb, val, children.next().unwrap().clone()) {
			Ok(t) => t,
			Err(e) => return Err(e)
		};
		bb = build_out.0;
		let val_type = match module.types.get(val.type_.clone() as usize) {
			Some(t) => t,
			None => return Err( "oops".to_string() )
		};
		let f = match get_op_fn(val_type, &build_out.1.type_,
		match c {
			'+' => "add",
			'-' => "sub",
			'*' => "mult",
			_ => ""
		}
		) {
			Ok(f) => f,
			Err((M, m)) => return Err(format!("{:0>2}{:0>2}", M, m))
		};
		let res = f(module, bb, val, build_out.1);
		bb = res.0;
		val = res.1;
		// 	None => return Err(format!("Type \"{}\" (ID: {}) does not implement adding shit", val_type.names.get("en").unwrap(), val.type_))
	}
	Ok((bb, val))
}

fn get_op_fn<'a>(self_type: &'a Type, other_id: &'a u16, name: &'a str) -> Result<&'a unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value), (u8, u8)> {
	let all_ops = match self_type.ops.get(name) {
		Some(map) => map,
		None => return Err((5, 3))
	};
	match all_ops.get(&other_id) {
		Some(f) => Ok(f),
		None => return Err((5, 4))
	}
}

unsafe fn build_if(module: &mut Module, mut bb: LLVMBasicBlockRef, mut val: Value, mut children: Vec<ASTNode>) -> Result<(LLVMBasicBlockRef, Value), String> {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	let mut else_node: Option<ASTNode> = None;
	if (children.len() % 2) != 0 {
		// Else present
		else_node = Some(children.pop().unwrap());
	}
	
	// Make the new block that'll be the end of the statement
	let final_block = LLVMAppendBasicBlock(module.current_fn, module.blank.as_ptr());
	
	let mut children_iter = children.iter();
	while let Some(node) = children_iter.next() {
		val = null_value();
		let res = match build_ast(module, bb, val, node.clone()) {
			Ok(t) => t,
			Err(e) => return Err(e)
		};
		let mut if_true = LLVMAppendBasicBlock(module.current_fn, module.blank.as_ptr());
		let if_false = LLVMAppendBasicBlock(module.current_fn, module.blank.as_ptr());
		LLVMBuildCondBr(builder, res.1.value, if_true, if_false);
		let body = children_iter.next().unwrap().clone();
		for ast in body.child_nodes {
			let res = match build_ast(module, if_true, val, ast) {
				Ok(t) => t,
				Err(e) => return Err(e)
			};
			if_true = res.0;
		}
		bb = if_false;
		LLVMPositionBuilderAtEnd(builder, if_true);
		LLVMBuildBr(builder, final_block);
		LLVMPositionBuilderAtEnd(builder, bb);
	}
	
	if let Some(node) = else_node {
		for ast in node.child_nodes {
			let res = match build_ast(module, bb, null_value(), ast) {
				Ok(t) => t,
				Err(e) => return Err(e)
			};
			bb = res.0;
		}
	}
	
	LLVMPositionBuilderAtEnd(builder, bb);
	LLVMBuildBr(builder, final_block);
	
	
	Ok((final_block, val))
}

unsafe fn build_comp_op(module: &mut Module, mut bb: LLVMBasicBlockRef, mut _val: Value, mut cmp_ops: Vec<char>, mut children: Vec<ASTNode>) -> Result<(LLVMBasicBlockRef, Value), String> {
	let builder = LLVMCreateBuilder();
	LLVMPositionBuilderAtEnd(builder, bb);
	// Build first pair of comparisons
	let res = match build_ast(module, bb, null_value(), children.pop().unwrap()) {
		Ok(t) => t,
		Err(e) => return Err(e)
	};
	bb = res.0;
	let rhs = res.1;
	let res = match build_ast(module, bb, null_value(), children.pop().unwrap()) {
		Ok(t) => t,
		Err(e) => return Err(e)
	};
	bb = res.0;
	let lhs = res.1;
	let val = Value::new(LLVMBuildICmp(
		builder,
		match cmp_ops.pop().unwrap() {
			'l' => LLVMIntPredicate::LLVMIntULT,
			'L' => LLVMIntPredicate::LLVMIntULE,
			'g' => LLVMIntPredicate::LLVMIntUGT,
			'G' => LLVMIntPredicate::LLVMIntUGE,
			'e' => LLVMIntPredicate::LLVMIntEQ,
			'n' => LLVMIntPredicate::LLVMIntNE,
			_ => unreachable!()
		},
		lhs.value, rhs.value, module.new_ptr_i8("")
	), 1);
	Ok((bb, val))
}

/// Builds a module to an object file
///
/// Takes an LLVMModule (`module: *mut LLVMModule`) and writes this module to an object file at the
/// given path
pub fn to_object_file(module: *mut LLVMModule, object_path: String) {
    let mut target = null_mut();
    let mut err_msg_ptr = null_mut();
    unsafe {
        let target_triple = LLVMGetTarget(module);
        LLVMGetTargetFromTriple(
			target_triple,
			&mut target,
			&mut err_msg_ptr
		);
        let cpu = CString::new("generic").unwrap();
        let features = CString::new("").unwrap();
        let target_machine = LLVMCreateTargetMachine(
			target,
			target_triple,
			cpu.as_ptr() as *const _,
			features.as_ptr() as *const _,
			LLVMCodeGenOptLevel::LLVMCodeGenLevelNone,
			LLVMRelocMode::LLVMRelocPIC,
			LLVMCodeModel::LLVMCodeModelDefault,
        );
        let file = CString::new(&*object_path).unwrap();
        let obj_err_raw = CString::new("").unwrap();
        let mut obj_error = obj_err_raw.as_ptr() as *mut i8;
        let res = LLVMTargetMachineEmitToFile(
            target_machine,
            module,
            file.as_ptr() as *mut i8,
            LLVMCodeGenFileType::LLVMObjectFile,
            &mut obj_error,
        );
        
        if res != 0 {
            println!("{}", CStr::from_ptr(obj_error as *const _).to_str().unwrap());
            std::process::exit(1);
        }
    }
}

/// convert the object file into an executable using clang
pub fn object_to_executable(path: String) {
    match std::process::Command::new("clang")
        .arg(format!("{}.o", &path))
        .arg("-o")
        .arg(&*path)
        .output() {
        Ok(res) => {
            if res.status.success() {
                let t = String::from_utf8_lossy(&res.stdout).to_string();
                if t.is_empty() {
                    println!("Ok")
                } else {
                    println!("Ok: {}", t);
                }
            } else {
                let t = String::from_utf8_lossy(&res.stderr).to_string();
                if t.is_empty() {
                    println!("Err")
                } else {
                    println!("Err: {}", t);
                }
            }
        }
        Err(e) => {
            println!("Oops... it didn't work sorry\n{}", e)
        }
    };
    
    // Delete object file
    std::fs::remove_file(format!("{}.o", path)).unwrap();
}
