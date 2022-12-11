use super::token::{Span, Token, TokenKind};
use super::Error;

#[derive(Debug)]
pub struct FoundToken {
    /// The index of the character __after__ the lexed token
    pub next_index: usize,
    /// Token lexed
    pub token: TokenKind,
}

/// Lex all tokens, if possible.
pub fn lex_to_end(source: &[char]) -> Result<Vec<Token>, Error> {
    let mut cursor = 0;
    let mut tokens = Vec::new();

    loop {
        cursor += lex_whitespace(&source[cursor..]);

        if cursor == source.len() {
            return Ok(tokens);
        }

        if let Some(FoundToken { token, next_index }) = lex_token(&source[cursor..]) {
            tokens.push(Token {
                span: Span::new(cursor, cursor + next_index),
                kind: token,
            });
            cursor += next_index;
        } else {
            return Err(Error { index: cursor });
        }
    }
}

/// Runs all lexers over supplied source, returning the first success
pub fn lex_token(source: &[char]) -> Option<FoundToken> {
    let lexers = [lex_number, lex_string, lex_keyword, lex_ident];

    for lexer in lexers {
        if let Some(ft) = lexer(source) {
            return Some(ft);
        }
    }

    None
}

/// Find the first token _after_ whitespace.
pub fn lex_whitespace(source: &[char]) -> usize {
    for (index, c) in source.iter().enumerate() {
        if !c.is_whitespace() {
            return index;
        }
    }

    source.len()
}

pub fn lex_number(source: &[char]) -> Option<FoundToken> {
    if source.is_empty() {
        return None;
    }

    {
        let s: String = source.iter().collect();

        if let Ok(n) = s.parse::<f64>() {
            return Some(FoundToken {
                token: TokenKind::Number(n),
                next_index: source.len(),
            });
        }
    }

    lex_number(&source[0..source.len() - 1])
}

fn is_ident_terminator(c: char) -> bool {
    c.is_whitespace() || "(){},;:[]".contains(c)
}

fn lex_ident(source: &[char]) -> Option<FoundToken> {
    let mut ident = String::new();

    for (index, c) in source.iter().enumerate() {
        if is_ident_terminator(*c) {
            return Some(FoundToken {
                next_index: index,
                token: TokenKind::Ident(ident),
            });
        } else {
            ident.push(*c)
        }
    }

    None
}

pub fn lex_string(source: &[char]) -> Option<FoundToken> {
    if *source.first()? != '\"' {
        return None;
    }

    let mut text = String::new();

    for (index, c) in source.iter().enumerate().skip(1) {
        if *c == '\"' {
            return Some(FoundToken {
                next_index: index + 1,
                token: TokenKind::String(text),
            });
        } else {
            text.push(*c);
        }
    }

    None
}

fn lex_characters(source: &[char], cs: &str, token: TokenKind) -> Option<FoundToken> {
    let sep: Vec<_> = cs.chars().collect();

    if source.get(0..cs.len())? == sep {
        Some(FoundToken {
            token,
            next_index: cs.len(),
        })
    } else {
        None
    }
}

macro_rules! lex_chars_to {
    ($($text:literal => $res:ident),*) => {
        fn lex_keyword(source: &[char]) -> Option<FoundToken> {
            $(
                if let Some(found) = lex_characters(source, $text, TokenKind::$res){
                    return Some(found);
                }
            )*

            None
        }
    };
}

lex_chars_to! {
    "(" => LeftParen,
    ")" => RightParen,
    "{" => LeftBrace,
    "}" => RightBrace,
    "[" => LeftBracket,
    "]" => RightBracket,
    "," => Comma,
    "==" => DoubleEquals,
    "=" => Equals,
    "+=" => AddEquals,
    "-=" => SubtractEquals,
    "*=" => MultiplyEquals,
    "/=" => DivideEquals,
    ">" => GreaterThan,
    "<" => LessThan,
    ":" => Colon,
    ";" => Semicolon,
    "+" => Plus,
    "-" => Minus,
    "*" => Asterisk,
    "/" => ForwardSlash,
    "true" => True,
    "false" => False,
    "let" => Let,
    "fn" => Fn,
    "while" => While,
    "return" => Return,
    "if" => If,
    "else" => Else
}
