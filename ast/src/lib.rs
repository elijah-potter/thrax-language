#![warn(clippy::pedantic)]

mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

pub type Program = Vec<Stmt>;
