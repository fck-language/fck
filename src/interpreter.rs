use crate::nodes::{*, ASTNodeType};
use crate::err_wrn::*;
use crate::bases::{Context, Position};
use crate::types::*;
use std::any::Any;
use std::ops::Deref;

// LLVM
use llvm_sys as llvm;
use llvm::core::*;
use llvm::execution_engine::*;
use llvm::target::*;
use std::mem;
use llvm_sys::prelude::*;

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

pub struct Interpreter {
    ast_vec: Vec<ASTNode>,
    current_node: Option<ASTNode>,
    context: LLVMContextRef,
}

impl Interpreter {
    pub unsafe fn new(ast_vec: Vec<ASTNode>) -> Interpreter {
        Interpreter { ast_vec, current_node: None , context: LLVMContextCreate()}
    }

    pub unsafe fn interpret(&mut self) {
        self.current_node = self.ast_vec.pop();
        while self.current_node.is_some() {
            let a = match self.find_func() {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            };
            self.current_node = self.ast_vec.pop();
        }
    }

    unsafe fn find_func(&mut self) -> Result<Type, Error> {
        match self.current_node.clone().unwrap().node_type {
            ASTNodeType::Int(_) | ASTNodeType::Float(_) | ASTNodeType::String(_) | ASTNodeType::Bool(_) => self.primitive(),
            // ASTNodeType::ArithOp(_) => self.arith_op(),
            // List => {}
            // VarAccess => {}
            // VarGetRange => {}
            // VarGetItem => {}
            // VarAssign => {}
            // VarReassign => {}
            // VarSubFunc => {}
            // MethodCall => {}
            // Ternary => {}
            // AsErrorCatch => {}
            // ASTNodeType::ArithOp => self.arith_op_node(),
            // CompOp => {}
            // UnaryNot => {}
            // UnaryPlus => {}
            // UnaryMinus => {}
            // Static => {}
            // As => {}
            // Range => {}
            // If => {}
            // Else => {}
            // AtName => {}
            // Case => {}
            // Option => {}
            // Iterate => {}
            // While => {}
            // FuncDef => {}
            // FuncArg => {}
            // Call => {}
            // Return => {}
            // Continue => {}
            // Break => {}
            _ => panic!(),
        }
    }

    unsafe fn primitive(&mut self) -> Result<Type, Error> {
        let node = self.current_node.clone().unwrap();
        Ok(Type::new(match node.node_type {
            ASTNodeType::Int(v) => LLVMConstInt(LLVMInt64Type(), v.unsigned_abs(), LLVM_TRUE),
            // ASTNodeType::Float(v) => LLVMFlo(LLVMFloatType(), v, LLVM_TRUE),
            _ => unreachable!()
        }, node.pos_start, node.pos_end))
    }

    // unsafe fn arith_op(&mut self) -> Result<Type, Error> {
    //     let node = self.current_node.clone().unwrap();
    //     let module = LLVMModuleCreateWithNameInContext(b"sum\0".as_ptr() as *const _, self.context);
    //     let builder = LLVMCreateBuilderInContext(self.context);
    //     let mut child_node_final: Vec<Type> = vec![];
    //     for i in node.child_nodes {
    //         self.current_node = Some(i);
    //         child_node_final.push(self.find_func()?)
    //     }
    //     let pos_start = child_node_final.get(0).unwrap().pos_start;
    //     let sum = child_node_final.pop().unwrap();
    //     let pos_end = sum.pos_end;
    //     let mut sum = sum.value;
    //     for i in child_node_final {
    //         sum = LLVMBuildAdd(builder, sum, i.value, b"sum\0".as_ptr() as *const _)
    //     }
    //     LLVMBuildRet(builder, sum);
    //     module.
    //     // LLVMDumpModule(builder);
    //
    //     Result::Ok(Type::new(sum, pos_start, pos_end))
    // }
    // fn arith_op_node(&mut self) -> Result<Type, Error> {
    //     let mut parent_node = self.current_node.clone().unwrap();
    //     let mut ops = parent_node.value.unwrap().chars().rev().collect::<String>();
    //     self.current_node = parent_node.child_nodes.pop();
    //     let mut out: dyn Any = match self.find_func() {
    //         Ok(v) => match v {
    //             Type::Primitive(v) => match v {
    //                 PrimitiveType::Float(v) => v,
    //                 PrimitiveType::Int(v) => v,
    //                 PrimitiveType::String(v) => v,
    //                 PrimitiveType::Bool(v) => v
    //             }
    //         },
    //         Err(e) => return Err(e)
    //     };
    //
    //     while ops.len() > 0 {
    //         self.current_node = parent_node.child_nodes.pop();
    //         let current_val = match self.find_func() {
    //             Ok(v) => v,
    //             Err(e) => return Err(e)
    //         };
    //         match ops.pop().unwrap() {
    //             '+' => out += current_val,
    //             '-' => out -= current_val,
    //             _ => panic!()
    //         };
    //     }
    //     Ok(out)
    // }

    // fn arith_op_node(&mut self) -> Result<Box<dyn Any>, Error> {
    //     let mut parent_node = self.current_node.clone().unwrap();
    //     self.current_node = parent_node.child_nodes.pop();
    //     let out = match self.find_func() {
    //         Ok(v) => v,
    //         Err(e) => return Err(e)
    //     };
    //     if out.is::<PrimitiveType>() {
    //         let out = out.downcast_ref::<PrimitiveType>().unwrap();
    //     } else {
    //         return Err(Error::new(self.current_node.unwrap().pos_start, self.current_node.unwrap().pos_end, 0u16, String::new()))
    //     }
    //     while parent_node.child_nodes.len() > 0 {
    //         self.current_node = parent_node.child_nodes.pop();
    //         let node = match self.find_func() {
    //             Ok(v) => v,
    //             Err(e) => return Err(e)
    //         };
    //         if node.is::<PrimitiveType>() {
    //             let node = node.downcast_ref::<PrimitiveType>().unwrap();
    //             let out = out
    //         }
    //     }
    //     return Ok(out)
    // }
}
