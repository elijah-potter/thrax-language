mod lex;
mod parse;

use ast::Stmt;
pub use lex::Error as LexError;
pub use parse::{Error as ParseError, ErrorKind as ParseErrorKind};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A problem was encountered while lexing: {0}")]
    Lex(#[from] LexError),
    #[error("A problem was encountered while parsing: {0}")]
    Parse(#[from] ParseError),
}
