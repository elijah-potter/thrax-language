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
    UndefinedAccess(String),
}
