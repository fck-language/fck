use crate::bases::Position;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNodeType {
    Int(i64),             // {value as a String}
    Float(f64),           // {value as a String}
    Bool(bool),            // {value as a String (0 = false, 1 = true)}
    String(String),          // {value as a String}
    List,            // None
    VarAccess(String),       // Variable identifier
    VarGetRange,     //
    VarGetItem,      //
    VarAssign(bool, u8, String),       // {return as int}{variable type from token excluding '1.'}{variable name}
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
    As(u8),              //
    Range,           // None
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
    Break(Option<String>)            // None
}

#[derive(Clone)]
pub struct ASTNode {
    pub(crate) node_type: ASTNodeType,
    pub(crate) child_nodes: Vec<ASTNode>,
    pub(crate) pos_start: Position,
    pub(crate) pos_end: Position,
}


impl ASTNode {
    pub fn new(node_type: ASTNodeType,
               child_nodes: Vec<ASTNode>,
               pos_start: Position,
               pos_end: Position) -> ASTNode {
        return ASTNode{node_type, child_nodes, pos_start, pos_end}
    }
    pub fn new_v(node_type: ASTNodeType,
               pos_start: Position,
               pos_end: Position,) -> ASTNode {
        return ASTNode { node_type, child_nodes: vec![], pos_start, pos_end}
    }
}

impl std::fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str(&*format!("{}{:?} [{} {}]\n", match self.child_nodes.len() {
            0 => '|',
            _ => '-'
        }, self.node_type, self.pos_start, self.pos_end));
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
