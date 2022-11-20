use ast::BinaryOpKind;

use crate::lex::{ShallowTokenKind, Token, TokenKind};
use crate::parse::Error;

pub trait TokensExt {
    fn get_token_kind<'a>(
        &'a self,
        index: usize,
        kind: ShallowTokenKind,
    ) -> Result<&'a Token, Error>;
    fn locate_first(&self, kind: ShallowTokenKind) -> Result<usize, Error>;
    fn get_binary_op(&self, index: usize) -> Result<BinaryOpKind, Error>;
}

impl TokensExt for [Token] {
    fn get_token_kind<'a>(
        &'a self,
        index: usize,
        kind: ShallowTokenKind,
    ) -> Result<&'a Token, Error> {
        let token = self
            .get(index)
            .ok_or(Error::expected_token(index, kind, None))?;

        if token.kind.as_shallow() == kind {
            Ok(token)
        } else {
            Err(Error::expected_token(index, kind, Some(token.clone())))
        }
    }

    fn locate_first(&self, kind: ShallowTokenKind) -> Result<usize, Error> {
        if self.len() == 0{
            return Err(Error::expected_token(0, kind, None))
        }

        self.iter()
            .enumerate()
            .find_map(|(index, token)| (token.kind.as_shallow() == kind).then_some(index))
            .ok_or(Error::expected_token(
                self.len() - 1,
                kind,
                self.last().cloned(),
            ))
    }

    fn get_binary_op(&self, index: usize) -> Result<BinaryOpKind, Error> {
        let token = self
            .get(index)
            .ok_or(Error::expected_binary_operator(index, None))?;

        let kind = match token.kind {
            TokenKind::Plus => BinaryOpKind::Add,
            TokenKind::Minus => BinaryOpKind::Subtract,
            TokenKind::Asterisk => BinaryOpKind::Multiply,
            TokenKind::ForwardSlash => BinaryOpKind::Divide,
            _ => return Err(Error::expected_binary_operator(index, Some(token.clone()))),
        };

        Ok(kind)
    }
}
