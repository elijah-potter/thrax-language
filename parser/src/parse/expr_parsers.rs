use std::collections::HashMap;

use ast::{BinaryOp, Expr, FnCall};

use super::common_parsers::parse_expr_list;
use super::tokens_ext::{LocatedBinaryOp, TokensExt};
use super::Error;
use crate::lex::{ShallowTokenKind, Token, TokenKind};

/// Runs all parsers over supplied source, returning the first success or last failure
pub fn parse_expr(tokens: &[Token]) -> Result<Expr, Error> {
    let parsers = [
        parse_binary_op,
        parse_fn_call,
        parse_single_token,
        parse_array_literal,
        parse_object_literal,
    ];

    let mut last_failure = None;

    for parser in parsers {
        match parser(tokens) {
            Ok(fe) => return Ok(fe),
            Err(err) => last_failure = Some(err),
        }
    }

    Err(last_failure.unwrap())
}

fn parse_single_token(tokens: &[Token]) -> Result<Expr, Error> {
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

    if tokens.len() != 1 {
        return Err(Error::failed_to_consume(1));
    }

    Ok(expr)
}

fn parse_binary_op(tokens: &[Token]) -> Result<Expr, Error> {
    // Iterate over binary ops
    // Check if possible to do: parse_expr()
    // If not, check next op
    let mut scan_start = 0;

    let (a, b, kind) = loop {
        let LocatedBinaryOp { op, location } = tokens.locate_first_binary_op(scan_start)?;

        // a + b
        let a_tokens = &tokens[..location];
        let Ok(a) = parse_expr(a_tokens) else{
            scan_start = location + 1;
            continue;
        };

        let b_tokens = &tokens[location + 1..];
        let Ok(b) = parse_expr(b_tokens) else{
            scan_start = location + 1;
            continue;
        };

        let consumed_token_count = a_tokens.len() + b_tokens.len() + 1;
        if consumed_token_count != tokens.len() {
            return Err(Error::failed_to_consume(consumed_token_count));
        }

        break (a, b, op);
    };

    Ok(Expr::BinaryOp(BinaryOp {
        kind,
        a: Box::new(a),
        b: Box::new(b),
    }))
}

fn parse_fn_call(tokens: &[Token]) -> Result<Expr, Error> {
    let identifier = tokens.get_token_kind(0, ShallowTokenKind::Ident)?;

    let found_list = parse_expr_list(
        &tokens[1..],
        ShallowTokenKind::Comma,
        ShallowTokenKind::LeftParen,
        ShallowTokenKind::RightParen,
    )
    .map_err(|err| err.relative_to(1))?;

    if found_list.next_index + 1 != tokens.len() {
        return Err(Error::failed_to_consume(found_list.next_index));
    }

    Ok(Expr::FnCall(FnCall {
        ident: identifier.clone().ident().unwrap(),
        args: found_list.iter_exprs().collect(),
    }))
}

fn parse_array_literal(tokens: &[Token]) -> Result<Expr, Error> {
    let found_list = parse_expr_list(
        tokens,
        ShallowTokenKind::Comma,
        ShallowTokenKind::LeftBracket,
        ShallowTokenKind::RightBracket,
    )?;

    if found_list.next_index != tokens.len() {
        return Err(Error::failed_to_consume(found_list.next_index));
    }

    Ok(Expr::ArrayLiteral(found_list.iter_exprs().collect()))
}

fn parse_object_literal(tokens: &[Token]) -> Result<Expr, Error> {
    let closing_index = tokens
        .locate_last_matched_right(ShallowTokenKind::LeftBrace, ShallowTokenKind::RightBrace)?;

    if closing_index == 1 {
        return Ok(Expr::ObjectLiteral(HashMap::new()));
    }

    if closing_index != tokens.len() - 1 {
        return Err(Error::failed_to_consume(closing_index));
    }

    let mut current_start = 1;
    let mut items = HashMap::new();
    let mut d = 0;

    while current_start < closing_index {
        let ident = tokens.get_token_kind(current_start, ShallowTokenKind::Ident)?;
        tokens.get_token_kind(current_start + 1, ShallowTokenKind::Colon)?;
        let current = &tokens[current_start + 2..closing_index - d];

        if current.is_empty() {
            return Err(Error::no_valid_expr(current_start + 3));
        }

        if d != 0
            && tokens
                .get_token_kind(closing_index - d, ShallowTokenKind::Comma)
                .is_err()
        {
            d += 1;
            continue;
        }

        match parse_expr(current) {
            Ok(expr) => {
                items.insert(ident.as_ident().unwrap().to_string(), expr);
                current_start += current.len() + 3;
                d = 0;
            }
            Err(_) => d += 1,
        }
    }

    Ok(Expr::ObjectLiteral(items))
}

#[cfg(test)]
mod tests {
    use super::{parse_array_literal, parse_binary_op};
    use crate::parse::expr_parsers::{parse_fn_call, parse_object_literal};
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

    #[test]
    fn parses_array_literal() {
        let tokens = tokenize("[a, b, \"test\", 23, [1, 2]]");

        let res = parse_array_literal(&tokens);

        res.unwrap();
    }

    #[test]
    fn parses_object_literal() {
        let tokens = tokenize("{ a: 1, b: 2, arr: [1, 2], str: \"test string\" }");

        let res = parse_object_literal(&tokens);

        res.unwrap();
    }
}
