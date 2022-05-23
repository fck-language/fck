use crate::prelude::{ Module, Value };
use phf::{Map, phf_map};
use llvm_sys::{
	prelude::{ LLVMBasicBlockRef, LLVMValueRef },
	core::*
};

/// Add functions for integer type
pub const INT_OPS_ADD: Map<
	u16, unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)
> = phf_map!{
	0u16 => unsafe { |module: &mut Module, bb: LLVMBasicBlockRef, self_val: Value, r_val: Value| -> (LLVMBasicBlockRef, Value) {
		let builder = LLVMCreateBuilder();
		LLVMPositionBuilderAtEnd(builder, bb);
		( bb, Value { value: LLVMBuildAdd(
			builder,
			self_val.value,
			r_val.value,
			module.blank.as_ptr()
		), type_: 0 })
	}}
};

/// Subtract functions for integer type
pub const INT_OPS_SUB: Map<
	u16, unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)
> = phf_map!{
	0u16 => unsafe { |module: &mut Module, bb: LLVMBasicBlockRef, self_val: Value, r_val: Value| -> (LLVMBasicBlockRef, Value) {
		let builder = LLVMCreateBuilder();
		LLVMPositionBuilderAtEnd(builder, bb);
		( bb, Value { value: LLVMBuildSub(
			builder,
			self_val.value,
			r_val.value,
			module.blank.as_ptr()
		), type_: 1 })
	}}
};

/// Subtract functions for integer type
pub const INT_OPS_MULT: Map<
	u16, unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)
> = phf_map!{
	0u16 => unsafe { |module: &mut Module, bb: LLVMBasicBlockRef, self_val: Value, r_val: Value| -> (LLVMBasicBlockRef, Value) {
		let builder = LLVMCreateBuilder();
		LLVMPositionBuilderAtEnd(builder, bb);
		( bb, Value { value: LLVMBuildMul(
			builder,
			self_val.value,
			r_val.value,
			module.blank.as_ptr()
		), type_: 0 })
	}}
};

/// Cast functions for integer type
pub const INT_OPS_CAST: Map<
	u16, unsafe fn(&mut Module, LLVMBasicBlockRef, Value, Value) -> (LLVMBasicBlockRef, Value)
> = phf_map!{
	0u16 => unsafe { |module: &mut Module, bb: LLVMBasicBlockRef, self_val: Value, r_val: Value| -> (LLVMBasicBlockRef, Value) {
		let builder = LLVMCreateBuilder();
		LLVMPositionBuilderAtEnd(builder, bb);
		( bb, Value { value: LLVMBuildAdd(
			builder,
			self_val.value,
			r_val.value,
			module.blank.as_ptr()
		), type_: 0 })
	}}
};
