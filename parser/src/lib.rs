mod lex;
mod parse;

use ast::Program;
pub use lex::{Error as LexError, Token, TokenKind};
pub use parse::{
    parse_stmt_list as parse_tokens, Error as ParseError, ErrorKind as ParseErrorKind,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("A problem was encountered while lexing: {0}")]
    Lex(#[from] LexError),
    #[error("A problem was encountered while parsing: {0}")]
    Parse(#[from] ParseError),
}

pub fn lex_string(source: &str) -> Result<Vec<Token>, LexError> {
    let seperated: Vec<_> = source.chars().collect();

    lex::lex_to_end(&seperated)
}

pub fn parse_string(source: &str) -> Result<Program, Error> {
    let tokens = lex_string(source)?;
    let program = parse::parse_stmt_list(&tokens)?;

    Ok(program)
}

#[cfg(test)]
mod test_utils {
    use crate::lex::{lex_to_end, Token};

    pub fn tokenize(source: &str) -> Vec<Token> {
        let chars: Vec<char> = source.chars().collect();
        lex_to_end(&chars).expect("Failed to tokenize.")
    }
}
