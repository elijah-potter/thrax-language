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

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Is)]
        pub enum ShallowTokenKind{
            $(
                $kind,
            )*
        }


    };
}

define_token_types! {
    Number(f64),
    String(String),
    Ident(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Equals,
    Let,
    Fn,
    While,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    True,
    False,
    Semicolon
}
