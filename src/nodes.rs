//! AST Nodes
//!
//! This file contains everything to do with AST nodes, including the different types of node
use crate::bases::Position;
use std::fmt::Formatter;

/// AST node types
///
/// These are all all the types an AST node can be, and contain type specific values
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNodeType {
    /// 64 bit integer type
    Int(i64),
    /// 64 bit float type
    Float(f64),
    /// Boolean type
    Bool(bool),
    /// String type
    String(String),
    /// List type
    List,
    /// Variable access. Holds the variable name(language formatted)
    VarAccess(String),
    VarGetRange,     //
    VarGetItem,      //
    /// Variable assignment. Holds if the variable should be returned, the variable type, and
    /// identifier(language formatted)
    VarAssign(bool, u16, String),
    VarReassign,     //
    VarSubFunc,      //
    MethodCall,      //
    Ternary(bool, bool),         //
    AsErrorCatch,    //
    ArithOp(Vec<char>),         // {single character operator identifier}*
                     // Double character operators like ** are turned into characters, ** = p (power) for example
    CompOp(Vec<char>),          // Same as ArithOp {and:& or:| not:!}
    UnaryNot,        // None
    UnaryPlus,       // None
    UnaryMinus,      // None
    Static,          //
    As(u16),              //
    Range,           // None
    /// If condition. Contains the label for the conditional
    If(Option<String>),              // Number of elif statements
    Else,            // Else or default node
    AtName,          //
    Case(Option<String>),            //
    Option,          //
    Iterate(Option<String>),         //
    While(Option<String>),           // None
    FuncDef(String),         // Function identifier
    FuncArg(String),         // Argument identifier
    Call,            //
    Return(bool),          // None
    Continue(Option<String>),        // None
    Break(Option<String>),
    /// Node to contain the body of a conditional or function
    Body
}

/// AST node
///
/// This is the intermediate between tokens and LLVM IR. We don't use the traditional left and right
/// node, but instead a `Vec<ASTNode>`, giving a unique way to work with ASTs
#[derive(Clone)]
pub struct ASTNode {
    /// AST node type
    pub(crate) node_type: ASTNodeType,
    /// Child nodes
    pub(crate) child_nodes: Vec<ASTNode>,
    /// Starting position of the AST
    pub(crate) pos_start: Position,
    /// Ending position of the AST
    pub(crate) pos_end: Position,
}


impl ASTNode {
    /// Create a new AST node
    pub fn new(node_type: ASTNodeType,
               child_nodes: Vec<ASTNode>,
               pos_start: Position,
               pos_end: Position) -> ASTNode {
        return ASTNode{node_type, child_nodes, pos_start, pos_end}
    }
    /// Create a new AST node with no children. Quality of life function
    pub fn new_v(node_type: ASTNodeType,
               pos_start: Position,
               pos_end: Position,) -> ASTNode {
        return ASTNode { node_type, child_nodes: vec![], pos_start, pos_end}
    }
}

impl std::fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pad = f.width().unwrap_or(0) + 1;
        let mut out = String::from(' ').repeat(pad - 1);
        
        out.push_str(
            &*format!("{}{:?} [{} {}]\n",
                      if self.child_nodes.is_empty() { '|' } else { '-' },
                      self.node_type, self.pos_start, self.pos_end
            )
        );
        if self.child_nodes.len() > 0 {
            self.child_nodes.iter().map(
                |n| out.push_str(&*format!("{:pad$?}", n))
            );
        }
        write!(f, "{}", out)
    }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}
