use ast::Expr;

use super::expr_parsers::parse_expr;
use super::tokens_ext::TokensExt;
use super::Error;
use crate::lex::{ShallowTokenKind, Token};

#[derive(Debug, Clone)]
pub struct FoundExprList {
    pub items: Vec<FoundExprListItem>,
    pub next_index: usize,
}

impl FoundExprList {
    /// Consuming iterator over constituent `Expr`s
    pub fn iter_exprs(self) -> impl Iterator<Item = Expr> {
        self.items.into_iter().map(|t| t.expr)
    }
}

#[derive(Debug, Clone)]
pub struct FoundExprListItem {
    pub expr: Expr,
    pub found_at: usize,
}

#[derive(Debug, Clone)]
pub struct FoundPropIdentList {
    pub prop_idents: Vec<String>,
    pub next_index: usize,
}

pub fn parse_expr_list(
    tokens: &[Token],
    separator: ShallowTokenKind,
    open: ShallowTokenKind,
    close: ShallowTokenKind,
) -> Result<FoundExprList, Error> {
    let closing_index = tokens.locate_last_matched_right(open, close)?;

    if closing_index == 1 {
        return Ok(FoundExprList {
            items: Vec::new(),
            next_index: 2,
        });
    }

    let mut current_start = 1;
    let mut items = Vec::new();
    let mut d = 0;

    while current_start < closing_index {
        let current = &tokens[current_start..closing_index - d];

        if current.is_empty() {
            return Err(Error::no_valid_expr(current_start));
        }

        if d != 0 && tokens.get_token_kind(closing_index - d, separator).is_err() {
            d += 1;
            continue;
        }

        match parse_expr(current) {
            Ok(expr) => {
                items.push(FoundExprListItem {
                    expr,
                    found_at: current_start,
                });
                current_start += current.len() + 1;
                d = 0;
            }
            Err(_) => d += 1,
        }
    }

    Ok(FoundExprList {
        items,
        next_index: closing_index + 1,
    })
}

/// Given "(a + b)", should return `[a, b]`
/// TODO: Use above function
pub fn parse_prop_ident_list(tokens: &[Token]) -> Result<FoundPropIdentList, Error> {
    let FoundExprList { items, next_index } = parse_expr_list(
        tokens,
        ShallowTokenKind::Comma,
        ShallowTokenKind::LeftParen,
        ShallowTokenKind::RightParen,
    )?;

    let mut prop_idents = Vec::new();

    for item in items {
        if let Some(ident) = item.expr.ident() {
            prop_idents.push(ident);
        } else {
            return Err(Error::expected_token(
                item.found_at,
                ShallowTokenKind::Ident,
                Some(tokens[item.found_at].clone()),
            ));
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
    use crate::lex::ShallowTokenKind;
    use crate::test_utils::tokenize;

    #[test]
    fn parses_empty_expr_list() {
        let tokens = tokenize("()");

        let res = parse_expr_list(
            &tokens,
            ShallowTokenKind::Comma,
            ShallowTokenKind::LeftParen,
            ShallowTokenKind::RightParen,
        );

        res.unwrap();
    }

    #[test]
    fn parses_prop_list() {
        let tokens = tokenize("(a, b, c)");

        let props = parse_prop_ident_list(&tokens).unwrap();

        assert_eq!(props.prop_idents, vec!["a", "b", "c"])
    }
}
