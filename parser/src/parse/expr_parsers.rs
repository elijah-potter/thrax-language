use ast::{BinaryOp, BinaryOpKind, Expr};

use super::tokens_ext::TokensExt;
use super::Error;
use crate::lex::{ShallowTokenKind, Token, TokenKind};

#[derive(Debug)]
pub struct FoundExpr {
    pub expression: Expr,
    pub next_index: usize,
}

/// Runs all parsers over supplied source, returning the first success or last failure
pub fn parse_expr(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let parsers = [parse_binary_op, parse_literal];

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
        expression: expr,
        next_index: 1,
    })
}

fn parse_binary_op(tokens: &[Token]) -> Result<FoundExpr, Error> {
    let FoundExpr {
        expression: a,
        next_index,
    } = parse_literal(tokens)?;

    let kind = tokens.get_binary_op(next_index)?;

    let FoundExpr {
        expression: b,
        next_index,
    } = parse_expr(&tokens[next_index + 1..]).map_err(|err| err.relative_to(1))?;

    Ok(FoundExpr {
        expression: Expr::BinaryOp(BinaryOp {
            kind,
            a: Box::new(a),
            b: Box::new(b),
        }),
        next_index,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_binary_op;
    use crate::lex::{lex_to_end, Token};
    use crate::parse::expr_parsers::parse_expr;

    fn tokenize(source: &str) -> Vec<Token> {
        let chars: Vec<char> = source.chars().collect();
        lex_to_end(&chars).expect("Failed to tokenize.")
    }

    #[test]
    fn parses_add() {
        let tokens = tokenize("1 + 23 / 2");

        let res = parse_binary_op(&tokens);

        dbg!(res.unwrap());
    }
}
