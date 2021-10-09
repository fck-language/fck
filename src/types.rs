use std::any::Any;
use std::ops::*;
use std::fmt::Formatter;
use crate::bases::Position;
use crate::err_wrn::Error;

pub enum Type {
    Primitive(PrimitiveType),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int(Int),
    Float(Float),
    Bool(Bool),
    String(String_)
}

#[derive(Debug)]
pub struct Int {
    pub pos_start: Position,
    pub pos_end: Position,
    pub value: i64,
    // pub built_in_traits: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17],
}

// pub const INT_FUNCS: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17] = [
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add
// ];

impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a, 'b> Add<&'b Int> for &'a Int {
    type Output = Int;

    fn add(self, other: &'b Int) -> Int {
        Int{
            pos_start: self.pos_start,
            pos_end: other.pos_end,
            value: self.value + other.value
        }
    }
}

#[derive(Debug)]
pub struct Float {
    pub pos_start: Position,
    pub pos_end: Position,
    pub value: f64,
    // pub built_in_traits: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17],
}

// pub const FLOAT_FUNCS: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17] = [
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add
// ];

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct Bool {
    pub pos_start: Position,
    pub pos_end: Position,
    pub value: bool,
    // pub built_in_traits: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17],
}

// pub const BOOL_FUNCS: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17] = [
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add
// ];

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct String_ {
    pub pos_start: Position,
    pub pos_end: Position,
    pub value: String,
    // pub built_in_traits: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17],
}

// pub const STRING_FUNCS: [fn(&mut PrimitiveType, Box<dyn Any>) -> Result<Box<dyn Any>, Error>; 17] = [
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add,
//     PrimitiveType::int_add
// ];

impl std::fmt::Display for String_ {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
