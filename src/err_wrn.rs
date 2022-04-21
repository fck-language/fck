use std::fmt::{Display, Formatter};
use crate::bases::Position;

#[derive(Clone)]
pub struct Error {
    pos_start: Position,
    pos_end: Position,
    error_index: u16
}

impl Error {
    pub fn new(pos_start: Position, pos_end: Position, error_index: u16) -> Error {
        Error{pos_start, pos_end, error_index}
    }
    
    pub fn show_position(&self, src: String) -> String {
        src.split('\n').collect::<Vec<&str>>().get(self.pos_start.ln..self.pos_end.ln + 1).unwrap().join("\n")
    }
}

impl Display for Error {
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
