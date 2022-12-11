use ast::BinaryOpKind;

use crate::value::ShallowValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A value as already been assigned to {0}.")]
    Redeclaration(String),
    #[error("Assignment to an undeclared variable {0}")]
    Undeclared(String),
    /// Represent that a type is being used where another type should.
    /// 0 => Should
    /// 1 => Used
    #[error("Attempted to use a {1} as a {0}.")]
    TypeError(ShallowValue, ShallowValue),
    #[error("Attempted to perform a `{:?}` operation with {0} and {1}. This is invalid.", .2)]
    InvalidBinaryOpArgs(ShallowValue, ShallowValue, BinaryOpKind),
    #[error("Attempted to access non-existant variable {0}.")]
    UndefinedStackAccess(String),
    #[error("Attempted to access non-existant heap item {0}.")]
    UndefinedHeapAccess(usize),
    /// Represent that function is being supplied too many arguments.
    /// 0 => # of args requested
    /// 1 => # of args supplied
    #[error("Function requires {1} arguments, but was supplied {0}")]
    IncorrectArgumentCount(usize, usize),
    #[error("Expected integer value, got {0}")]
    ExpectedInteger(f64),
    #[error("Requested string or integer index {0} is out of bounds.")]
    IndexOutOfBounds(usize),
    #[error("Requested key {0} in object does not exist in object.")]
    ObjectMissingKey(String),
    #[error("Requestion type {0} cannot be indexed.")]
    CannotIndexType(ShallowValue),
}
