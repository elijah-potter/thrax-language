use ast::BinaryOpKind;

use crate::lex::{ShallowTokenKind, Token};
use crate::parse::Error;
use crate::TokenKind;

pub struct LocatedBinaryOp {
    pub op: BinaryOpKind,
    pub location: usize,
}

pub trait TokensExt {
    fn get_token_kind<'a>(
        &'a self,
        index: usize,
        kind: ShallowTokenKind,
    ) -> Result<&'a TokenKind, Error>;
    fn locate_first(&self, starting_at: usize, kind: ShallowTokenKind) -> Result<usize, Error>;
    fn get_binary_op(&self, index: usize) -> Result<BinaryOpKind, Error>;
    fn locate_last_matched_right(
        &self,
        left: ShallowTokenKind,
        right: ShallowTokenKind,
    ) -> Result<usize, Error>;
    fn locate_first_binary_op(&self, starting_at: usize) -> Result<LocatedBinaryOp, Error>;
}

impl TokensExt for [Token] {
    fn get_token_kind<'a>(
        &'a self,
        index: usize,
        kind: ShallowTokenKind,
    ) -> Result<&'a TokenKind, Error> {
        let token = self
            .get(index)
            .ok_or(Error::expected_token(index, kind, None))?;

        if token.kind.as_shallow() == kind {
            Ok(&token.kind)
        } else {
            Err(Error::expected_token(index, kind, Some(token.clone())))
        }
    }

    fn locate_first(&self, starting_at: usize, kind: ShallowTokenKind) -> Result<usize, Error> {
        if self.is_empty() {
            return Err(Error::expected_token(0, kind, None));
        }

        self.iter()
            .enumerate()
            .skip(starting_at)
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

        token
            .kind
            .as_binary_op()
            .ok_or(Error::expected_binary_operator(index, Some(token.clone())))
    }

    /// Meant for brackets.
    ///
    /// For example, finding the final `]` in `[a + [b]]`.
    ///
    /// First token must be `left`
    ///
    /// Returns an error for expected `right` if not found.
    fn locate_last_matched_right(
        &self,
        left: ShallowTokenKind,
        right: ShallowTokenKind,
    ) -> Result<usize, Error> {
        self.get_token_kind(0, left)?;

        let mut left_count = 1;

        for (index, token) in self.iter().enumerate().skip(1) {
            if token.kind.as_shallow() == left {
                left_count += 1
            } else if token.kind.as_shallow() == right {
                if left_count == 1 {
                    return Ok(index);
                } else {
                    left_count -= 1;
                }
            }
        }

        Err(Error::expected_token(self.len() - 1, right, None))
    }

    fn locate_first_binary_op(&self, starting_at: usize) -> Result<LocatedBinaryOp, Error> {
        if self.is_empty() {
            return Err(Error::expected_binary_operator(0, None));
        }

        for (index, token) in self.iter().enumerate().skip(starting_at) {
            if let Some(op) = token.kind.as_binary_op() {
                return Ok(LocatedBinaryOp {
                    op,
                    location: index,
                });
            }
        }

        Err(Error::expected_binary_operator(
            self.len() - 1,
            self.last().cloned(),
        ))
    }
}
