//! fck primitive types
//!
//! The primitives that make up the rest of fck
use crate::{
	prelude::{ Type, Value },
};
use phf::{Map, phf_map};
use llvm_sys::{
	core::*
};

/// 64 bit integer type
///
/// Type ID 1
pub const INT: Type = Type {
	names: phf_map!{
		"en" => "int"
	},
	ops: phf_map!{
		"add" => int::INT_OPS_ADD,
		"sub" => int::INT_OPS_SUB,
		"mult" => int::INT_OPS_MULT,
		"cast" => int::INT_OPS_CAST
	},
	functions: phf_map!{},
	llvm_type: unsafe { || { LLVMInt64Type() }}
};

pub const NULL_TYPE: Type = Type {
	names: phf_map!{
		"en" => "null"
	},
	ops: phf_map!{},
	functions: phf_map!{},
	llvm_type: unsafe { || { LLVMInt1Type() }}
};

pub fn null_value() -> Value {
	Value {
		value: unsafe { LLVMConstNull(LLVMInt1Type()) },
		type_: 0
	}
}
