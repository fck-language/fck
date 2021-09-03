use crate::bases::Position;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ASTNodeType {
    Int,             // {value as a String}
    Float,           // {value as a String}
    Bool,            // {value as a String (0 = false, 1 = true)}
    String,          // {value as a String}
    List,            // None
    VarAccess,       // Variable identifier
    VarGetRange,     //
    VarGetItem,      //
    VarAssign,       // {return as int}{variable type from token excluding '1.'}{variable name}
    VarReassign,     //
    VarSubFunc,      //
    MethodCall,      //
    TrueFalse,       //
    AssErrorCatch,   //
    BinOp,           //
    UnaryOp,         //
    Static,          //
    As,              //
    If,              // None
    AtName,          //
    Case,            //
    Option,          //
    Iterate,         //
    While,           // None
    FuncDef,         // Function identifier
    FuncArg,         // Argument identifier
    Call,            //
    Return,          // None
    Continue,        // None
    Break            // None
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
