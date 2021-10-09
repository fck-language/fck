use crate::nodes::{*, ASTNodeType};
use crate::err_wrn::*;
use crate::bases::{Context, Position};
use crate::types::*;
use std::any::Any;
use std::ops::Deref;
use std::hint::unreachable_unchecked;

pub struct Interpreter {
    ast_vec: Vec<ASTNode>,
    current_node: Option<ASTNode>,
}

impl Interpreter {
    pub fn new(ast_vec: Vec<ASTNode>) -> Interpreter {
        Interpreter { ast_vec, current_node: None }
    }

    pub fn interpret(&mut self) {
        self.current_node = self.ast_vec.pop();
        while self.current_node.is_some() {
            let a = match self.find_func() {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            };
            match a {
                Type::Primitive(p) => println!("{:?}", p)
            }
            self.current_node = self.ast_vec.pop();
        }
    }

    fn find_func(&mut self) -> Result<Type, Error> {
        match self.current_node.clone().unwrap().node_type {
            ASTNodeType::Int(_) | ASTNodeType::Float(_) | ASTNodeType::String(_) | ASTNodeType::Bool(_) => self.primitive(),
            // Bool => {}
            // String => {}
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

    fn primitive(&mut self) -> Result<Type, Error> {
        let node = self.current_node.clone().unwrap();
        Ok(Type::Primitive(match node.node_type {
            ASTNodeType::Int(v) => PrimitiveType::Int(Int{
                pos_start: self.current_node.clone().unwrap().pos_start,
                pos_end: self.current_node.clone().unwrap().pos_end,
                value: v
            }),
            ASTNodeType::Float(v) => PrimitiveType::Float(Float{
                pos_start: self.current_node.clone().unwrap().pos_start,
                pos_end: self.current_node.clone().unwrap().pos_end,
                value: v
            }),
            ASTNodeType::String(v) => PrimitiveType::String(String_{
                pos_start: self.current_node.clone().unwrap().pos_start,
                pos_end: self.current_node.clone().unwrap().pos_end,
                value: v
            }),
            ASTNodeType::Bool(v) => PrimitiveType::Bool(Bool{
                pos_start: self.current_node.clone().unwrap().pos_start,
                pos_end: self.current_node.clone().unwrap().pos_end,
                value: v
            }),
            _ => unreachable!()
        }))
    }

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
