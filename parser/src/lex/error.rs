use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    pub index: usize,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer did not expect character at index {}", self.index)
    }
}
