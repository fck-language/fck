use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Error {

}

impl Error {
    pub fn new() -> Error {
        return Error{}
    }
}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Warning {

}

impl Warning {

}

impl Display for Warning {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
