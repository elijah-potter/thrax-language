use ast::BinaryOpKind;
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
            #[must_use] pub fn as_shallow(&self) -> ShallowTokenKind{
                match self{
                    $(
                        Self::$kind$((::paste::paste!{[<_$contains:snake>]}))? => ShallowTokenKind::$kind,
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
    };
}

impl TokenKind {
    #[must_use]
    pub fn as_binary_op(&self) -> Option<BinaryOpKind> {
        match self {
            TokenKind::Plus => Some(BinaryOpKind::Add),
            TokenKind::Minus => Some(BinaryOpKind::Subtract),
            TokenKind::Asterisk => Some(BinaryOpKind::Multiply),
            TokenKind::ForwardSlash => Some(BinaryOpKind::Divide),
            TokenKind::DoubleEquals => Some(BinaryOpKind::Equals),
            TokenKind::GreaterThan => Some(BinaryOpKind::GreaterThan),
            TokenKind::LessThan => Some(BinaryOpKind::LessThan),
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
    GreaterThan,
    LessThan,
    Let,
    Fn,
    Return,
    While,
    If,
    Else,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    True,
    False,
    Colon,
    Semicolon
}
