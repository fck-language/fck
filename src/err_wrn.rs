use std::fmt::{Display, Formatter, Debug};
use crate::bases::Position;

/// Error struct
#[derive(Clone)]
pub struct Error {
    pos_start: Position,
    pos_end: Position,
    pub error_index: u16
}

impl Error {
    /// Make a new error instance
    pub fn new(pos_start: Position, pos_end: Position, error_index: u16) -> Error {
        Error{ pos_start, pos_end, error_index }
    }
    
    /// Returns the line(s) the error is referencing
    pub fn show_position(&self, src: String) -> String {
        src.split('\n').collect::<Vec<&str>>().get(self.pos_start.ln..self.pos_end.ln + 1).unwrap().join("\n")
    }
}

/// Display the error
///
/// this will probably be changed later on
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(E{:04}) [{} -> {}]", self.error_index, self.pos_start, self.pos_end)
    }
}

/// Debug for the error
///
/// Only actually used by the testing environment
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(E{:04}) [{} -> {}]", self.error_index, self.pos_start, self.pos_end)
    }
}

#[derive(Clone)]
pub struct Warning {

}

impl Warning {

}

impl Display for Warning {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
