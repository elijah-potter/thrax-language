use std::fmt::{Display, Formatter};

use is_macro::Is;

use crate::lex::{ShallowTokenKind, Token};

#[derive(Debug)]
pub struct Error {
    // TODO: Make this a span
    pub index: usize,
    pub kind: ErrorKind,
    // Whether the error can be recovered from.
    //
    // For example, a parser of a while loop can return recoverable if the `while` token isn't found,
    // but not if the boolean expression associated with it is malformed.
    //
    // Additionally, if this is `false` the error should always be pushed to the top.
    pub is_recoverable: bool,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Is)]
pub enum ErrorKind {
    ExpectedToken {
        expected: ShallowTokenKind,
        received: Option<Token>,
    },
    ExpectedBinaryOperator {
        received: Option<Token>,
    },
    ExpectedAssignmentOperator {
        received: Option<Token>,
    },
    ExpectedLiteral {
        received: Option<Token>,
    },
    FailedToConsume,
    NoValidExpr,
    NoTokensProvided,
}

impl Error {
    /// Adjusts [`Self::index`] by an index.
    pub fn offset(mut self, by: usize) -> Self {
        self.index += by;
        self
    }

    /// Sets the consumed [`Self::is_recoverable`] to `false`
    pub fn unrecoverable(mut self) -> Self {
        self.is_recoverable = false;
        self
    }

    pub fn expected_token(
        at_index: usize,
        expected: ShallowTokenKind,
        received: Option<Token>,
    ) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedToken { expected, received },
            is_recoverable: true,
        }
    }

    pub fn expected_binary_operator(at_index: usize, received: Option<Token>) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedBinaryOperator { received },
            is_recoverable: true,
        }
    }

    pub fn expected_assignment_operator(at_index: usize, received: Option<Token>) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedAssignmentOperator { received },
            is_recoverable: true,
        }
    }

    pub fn expected_literal(at_index: usize, received: Option<Token>) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::ExpectedLiteral { received },
            is_recoverable: true,
        }
    }

    pub fn failed_to_consume(at_index: usize) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::FailedToConsume,

            is_recoverable: true,
        }
    }

    pub fn no_valid_expr(at_index: usize) -> Self {
        Self {
            index: at_index,
            kind: ErrorKind::NoValidExpr,
            is_recoverable: true,
        }
    }

    pub fn no_tokens_provided() -> Self {
        Self {
            index: 0,
            kind: ErrorKind::NoTokensProvided,
            is_recoverable: true,
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::ExpectedToken { expected, received } => {
                write!(f, "Expected {expected} got ")?;
                match received {
                    Some(token) => write!(f, "{}", token.kind),
                    None => write!(f, "to the end of the buffer."),
                }
            }
            ErrorKind::ExpectedBinaryOperator { received } => {
                write!(f, "Expected binary operator got ")?;
                match received {
                    Some(token) => write!(f, "{}", token.kind),
                    None => write!(f, "to the end of the buffer."),
                }
            }
            ErrorKind::ExpectedAssignmentOperator { received } => {
                write!(f, "Expected assignment operator got ")?;
                match received {
                    Some(token) => write!(f, "{}", token.kind),
                    None => write!(f, "to the end of the buffer."),
                }
            }
            ErrorKind::ExpectedLiteral { received } => {
                write!(f, "Expected literal got ")?;
                match received {
                    Some(token) => write!(f, "{}", token.kind),
                    None => write!(f, "to the end of the buffer."),
                }
            }
            ErrorKind::FailedToConsume => write!(f, "Failed to consume the provided input."),
            ErrorKind::NoValidExpr => write!(f, "No valid expression was found."),
            ErrorKind::NoTokensProvided => write!(f, "No tokens were provided."),
        }
    }
}
