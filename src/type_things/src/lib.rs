//! fck type handling system
//!
//! This contains the generic `type`, all the functions for primitive types, as well as a type
//! checker that is run (by default but can be turned off for speed) before compilation
pub mod checker;
pub mod primitives;
pub mod prelude;
mod int;
pub mod symbol_tables;
