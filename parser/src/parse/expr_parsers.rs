use ast::{BinaryOp, BinaryOpKind, Expr, FnCall};

use super::common_parsers::{parse_expr_list, FoundExprList};
use super::tokens_ext::{LocatedBinaryOp, TokensExt};
use super::Error;
use crate::lex::{ShallowTokenKind, Token, TokenKind};

#[derive(Debug)]
pub struct FoundExpr {
    pub expr: Expr,
    pub next_index: usize,
}

/// Runs all parsers over supplied source, returning the first success or last failure
pub fn parse_expr(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let parsers = [parse_fn_call, parse_binary_op, parse_literal];

    let mut last_failure = None;

    for parser in parsers {
        match parser(tokens) {
            Ok(fe) => return Ok(fe),
            Err(err) => last_failure = Some(err),
        }
    }

    Err(last_failure.unwrap())
}

fn parse_literal(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let token = tokens
        .get(0)
        .ok_or(Error::expected_literal(0, None))?
        .clone();

    let expr = match token.kind {
        TokenKind::Number(n) => Expr::NumberLiteral(n),
        TokenKind::String(s) => Expr::StringLiteral(s),
        TokenKind::Ident(i) => Expr::Ident(i),
        TokenKind::True => Expr::BoolLiteral(true),
        TokenKind::False => Expr::BoolLiteral(false),
        _ => return Err(Error::expected_literal(0, Some(token))),
    };

    Ok(FoundExpr {
        expr,
        next_index: 1,
    })
}

fn parse_binary_op(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let LocatedBinaryOp { op: kind, location } = tokens.locate_first_binary_op()?;

    let FoundExpr { expr: a, .. } = parse_expr(&tokens[..location])?;

    let FoundExpr {
        expr: b,
        next_index,
    } = parse_expr(&tokens[location + 1..]).map_err(|err| err.relative_to(location + 1))?;

    Ok(FoundExpr {
        expr: Expr::BinaryOp(BinaryOp {
            kind,
            a: Box::new(a),
            b: Box::new(b),
        }),
        next_index: next_index + location + 1,
    })
}

fn parse_fn_call(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let identifier = tokens.get_token_kind(0, ShallowTokenKind::Ident)?;

    let FoundExprList { exprs, next_index } =
        parse_expr_list(&tokens[1..], ShallowTokenKind::Comma).map_err(|err| err.relative_to(1))?;

    Ok(FoundExpr {
        expr: Expr::FnCall(FnCall {
            ident: identifier.kind.clone().ident().unwrap(),
            args: exprs,
        }),
        next_index: next_index + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_binary_op;
    use crate::parse::expr_parsers::parse_fn_call;
    use crate::test_utils::tokenize;

    // TODO: ADD WAY MORE TESTS

    #[test]
    fn parses_add() {
        let tokens = tokenize("1 + 23 / 2");

        let res = parse_binary_op(&tokens);

        res.unwrap();
    }

    #[test]
    fn parses_fn_call() {
        let tokens = tokenize("test(a + 12, b)");

        let res = parse_fn_call(&tokens);

        res.unwrap();
    }
}
