use ast::Expr;

use super::expr_parsers::{parse_expr, FoundExpr};
use super::tokens_ext::TokensExt;
use super::Error;
use crate::lex::{ShallowTokenKind, Token};

#[derive(Debug, Clone)]
pub struct FoundExprList {
    pub exprs: Vec<Expr>,
    pub next_index: usize,
}

#[derive(Debug, Clone)]
pub struct FoundPropIdentList {
    pub prop_idents: Vec<String>,
    pub next_index: usize,
}

pub fn parse_expr_list(
    tokens: &[Token],
    seperator: ShallowTokenKind,
) -> Result<FoundExprList, Error> {
    let closing_paren_index = tokens
        .locate_last_matched_right(ShallowTokenKind::LeftParen, ShallowTokenKind::RightParen)?;

    let mut items = Vec::new();

    let mut current = 1;

    while let Ok(sep_index) = tokens[current..closing_paren_index].locate_first(seperator) {
        let FoundExpr {
            expr: expression, ..
        } = parse_expr(&tokens[current..][..sep_index]).map_err(|err| err.relative_to(current))?;
        items.push(expression);
        current = current + sep_index + 1;
    }

    if let Ok(FoundExpr {
        expr: expression, ..
    }) =
        parse_expr(&tokens[current..closing_paren_index]).map_err(|err| err.relative_to(current))
    {
        items.push(expression);
    }

    Ok(FoundExprList {
        exprs: items,
        next_index: closing_paren_index + 1,
    })
}

/// Given "(a + b)", should return `[a, b]`
/// TODO: Use above function
pub fn parse_prop_ident_list(tokens: &[Token]) -> Result<FoundPropIdentList, Error> {
    let FoundExprList { exprs, next_index } = parse_expr_list(tokens, ShallowTokenKind::Comma)?;

    let mut prop_idents = Vec::new();

    for expr in exprs {
        if let Some(ident) = expr.ident() {
            prop_idents.push(ident);
        } else {
            // TODO: Revise this after adding span to errors
            return Err(Error::expected_token(0, ShallowTokenKind::Ident, None));
        }
    }

    Ok(FoundPropIdentList {
        prop_idents,
        next_index,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_expr_list, parse_prop_ident_list};
    use crate::test_utils::tokenize;

    #[test]
    fn parses_empty_expr_list() {
        let tokens = tokenize("()");

        let res = parse_expr_list(&tokens, crate::lex::ShallowTokenKind::Comma);

        res.unwrap();
    }

    #[test]
    fn parses_prop_list() {
        let tokens = tokenize("(a, b, c)");

        let props = parse_prop_ident_list(&tokens).unwrap();

        assert_eq!(props.prop_idents, vec!["a", "b", "c"])
    }
}
