use std::fmt::{Display, Formatter};

use ast::{AssignOpKind, BinaryOpKind};
use is_macro::Is;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

macro_rules! define_token_types {
    ($($kind:ident$(($contains:ty))?),*) => {
        #[derive(Debug, Clone, PartialEq, Is)]
        pub enum TokenKind{
            $(
                $kind $(($contains))?,
            )*
        }

        impl TokenKind {
             pub fn as_shallow(&self) -> ShallowTokenKind{
                match self{
                    $(
                        Self::$kind$((::paste::paste!{[<_$contains:snake>]}))? => ShallowTokenKind::$kind,
                    )*
                }
            }
        }

        impl Display for TokenKind{
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$kind$((::paste::paste!{[<_$contains:snake>]}))? => {
                            write!(f, "{}", stringify!($kind))?;
                            $(write!(f, ": {}", ::paste::paste!{[<_$contains:snake>]})?;)?
                            Ok(())
                        },
                    )*
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Is)]
        pub enum ShallowTokenKind{
            $(
                $kind,
            )*
        }

        impl Display for ShallowTokenKind{
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$kind => write!(f, stringify!($kind)),
                    )*
                }
            }
        }
    };
}

impl TokenKind {
    pub fn as_binary_op(&self) -> Option<BinaryOpKind> {
        match self {
            TokenKind::Plus => Some(BinaryOpKind::Add),
            TokenKind::Minus => Some(BinaryOpKind::Subtract),
            TokenKind::Asterisk => Some(BinaryOpKind::Multiply),
            TokenKind::ForwardSlash => Some(BinaryOpKind::Divide),
            TokenKind::DoubleEquals => Some(BinaryOpKind::Equals),
            TokenKind::GreaterThan => Some(BinaryOpKind::GreaterThan),
            TokenKind::DoubleAsterisk => Some(BinaryOpKind::Pow),
            TokenKind::LessThan => Some(BinaryOpKind::LessThan),
            _ => None,
        }
    }

    pub fn as_assign_op(&self) -> Option<AssignOpKind> {
        match self {
            TokenKind::Equals => Some(AssignOpKind::NoOp),
            TokenKind::AddEquals => Some(AssignOpKind::Op(BinaryOpKind::Add)),
            TokenKind::SubtractEquals => Some(AssignOpKind::Op(BinaryOpKind::Subtract)),
            TokenKind::MultiplyEquals => Some(AssignOpKind::Op(BinaryOpKind::Multiply)),
            TokenKind::DivideEquals => Some(AssignOpKind::Op(BinaryOpKind::Divide)),
            _ => None,
        }
    }
}

define_token_types! {
    Number(f64),
    String(String),
    Ident(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Equals,
    DoubleEquals,
    AddEquals,
    SubtractEquals,
    MultiplyEquals,
    DivideEquals,
    GreaterThan,
    LessThan,
    Let,
    Fn,
    Return,
    Break,
    Continue,
    While,
    If,
    Else,
    Plus,
    Minus,
    Asterisk,
    DoubleAsterisk,
    ForwardSlash,
    True,
    False,
    Colon,
    Semicolon
}
