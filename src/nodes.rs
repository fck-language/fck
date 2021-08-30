use crate::bases::{Token, Position};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ASTNodeType {
    IntNode,
    FloatNode,
    Bool,
    String,
    List,
    VarAccess,
    VarGetRange,
    VarGetItem,
    VarAssign,
    VarReassign,
    VarSubFunc,
    MethodCall,
    TrueFalse,
    AssErrorCatch,
    BinOp,
    UnaryOp,
    Silent,
    As,
    If,
    AtName,
    Case,
    Option,
    Iterate,
    While,
    FuncDef,
    FuncArg,
    Call,
    Return,
    Continue,
    Break
}

pub struct ASTNode {
    pub(crate) node_type: ASTNodeType,
    pub(crate) child_nodes: Vec<ASTNode>,
    pub(crate) pos_start: Position,
    pub(crate) pos_end: Position,
    pub(crate) value: Option<String>
}


impl ASTNode {
    pub fn new(node_type: ASTNodeType,
               child_nodes: Vec<ASTNode>,
               pos_start: Position,
               pos_end: Position,
               value: Option<String>) -> ASTNode {
        return ASTNode{node_type, child_nodes, pos_start, pos_end, value}
    }
}

impl std::fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str(&*format!("{}{:?} [{}-{}, {:?}]\n", match self.child_nodes.len() {
            0 => '|',
            _ => '-'
        }, self.node_type, self.pos_start, self.pos_end, self.value));
        if self.child_nodes.len() > 0 {
            for node in self.child_nodes.iter() {
                out.push_str(&*format!("{:?}", node).replace("|", " |").replace("-", " -"));
            }
        }
        write!(f, "{}", out)
    }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}
