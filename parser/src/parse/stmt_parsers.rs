use ast::{Expr, FnCall, FnDecl, Stmt, VarAssign, VarDecl};

use super::expr_parsers::{parse_expr, FoundExpr};
use super::tokens_ext::TokensExt;
use super::Error;
use crate::lex::{ShallowTokenKind, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct FoundStmt {
    pub stmt: Stmt,
    pub next_index: usize,
}

#[derive(Debug, Clone)]
struct FoundPropIdentList {
    pub prop_idents: Vec<String>,
    pub next_index: usize,
}

#[derive(Debug, Clone)]
struct FoundExprList {
    pub exprs: Vec<Expr>,
    pub next_index: usize,
}

pub fn parse_stmt_list(tokens: &[Token]) -> Result<Vec<Stmt>, Error> {
    let mut stmts = Vec::new();

    let mut current_index = 0;

    while (current_index) < (tokens.len()) {
        let FoundStmt { stmt, next_index } =
            parse_stmt(&tokens[current_index..]).map_err(|err| err.relative_to(current_index))?;

        current_index += next_index;
        stmts.push(stmt);
    }

    Ok(stmts)
}

/// Runs all parsers over supplied source, returning the first success or last failure
pub fn parse_stmt(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let parsers = [parse_var_decl, parse_var_assign, parse_fn_decl, parse_fn_call, parse_while_loop];

    let mut last_failure = None;

    for parser in parsers {
        match parser(tokens) {
            Ok(fe) => return Ok(fe),
            Err(err) => last_failure = Some(err),
        }
    }

    Err(last_failure.unwrap())
}

fn parse_var_decl(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::Let)?;

    let identifier = tokens.get_token_kind(1, ShallowTokenKind::Ident)?;

    tokens.get_token_kind(2, ShallowTokenKind::Equals)?;

    let semi_location = tokens.locate_first(ShallowTokenKind::Semicolon)?;

    let FoundExpr {
        expression,
        next_index,
    } = parse_expr(&tokens[3..semi_location]).map_err(|err| err.relative_to(3))?;

    Ok(FoundStmt {
        stmt: Stmt::VarDecl(VarDecl {
            ident: identifier.kind.clone().ident().unwrap(),
            initializer: expression,
        }),
        next_index: semi_location + 1,
    })
}

fn parse_var_assign(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let identifier = tokens.get_token_kind(0, ShallowTokenKind::Ident)?;

    tokens.get_token_kind(2, ShallowTokenKind::Equals)?;

    let semi_location = tokens.locate_first(ShallowTokenKind::Semicolon)?;

    let FoundExpr {
        expression,
        next_index,
    } = parse_expr(&tokens[3..semi_location]).map_err(|err| err.relative_to(3))?;

    Ok(FoundStmt {
        stmt: Stmt::VarAssign(VarAssign {
            ident: identifier.kind.clone().ident().unwrap(),
            value: expression,
        }),
        next_index: semi_location + 1,
    })
}

fn parse_fn_decl(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::Fn)?;

    let identifier = tokens.get_token_kind(1, ShallowTokenKind::Ident)?;

    let FoundPropIdentList {
        prop_idents,
        next_index,
    } = parse_prop_ident_list(&tokens[2..]).map_err(|err| err.relative_to(2))?;

    let closing_brace_index = locate_last_matched_right(
        &tokens[next_index + 2..],
        ShallowTokenKind::LeftBrace,
        ShallowTokenKind::RightBrace,
    )
    .map_err(|err| err.relative_to(next_index + 2))?;

    let body = parse_stmt_list(&tokens[next_index + 3..][..closing_brace_index - 1])
        .map_err(|err| err.relative_to(next_index + 3))?;

    Ok(FoundStmt {
        stmt: Stmt::FnDecl(FnDecl {
            ident: identifier.kind.clone().ident().unwrap(),
            prop_idents,
            body,
        }),
        next_index: closing_brace_index + 1,
    })
}

fn parse_fn_call(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let identifier = tokens.get_token_kind(0, ShallowTokenKind::Ident)?;

    let FoundExprList { exprs, next_index } =
        parse_expr_list(&tokens[1..], ShallowTokenKind::Comma).map_err(|err| err.relative_to(1))?;

    tokens.get_token_kind(next_index + 1, ShallowTokenKind::Semicolon)?;

    Ok(FoundStmt {
        stmt: Stmt::FnCall(FnCall {
            ident: identifier.kind.clone().ident().unwrap(),
            args: exprs,
        }),
        next_index: next_index + 2,
    })
}

fn parse_while_loop(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::While)?;

    let closing_paren_index = locate_last_matched_right(
        &tokens[1..],
        ShallowTokenKind::LeftParen,
        ShallowTokenKind::RightParen,
    )? + 1;

    let FoundExpr {
        expression,
        next_index,
    } = parse_expr(&tokens[2..closing_paren_index])?;

    let closing_brace_index = locate_last_matched_right(
        &tokens[closing_paren_index + 1..],
        ShallowTokenKind::LeftBrace,
        ShallowTokenKind::RightBrace,
    )
    .map_err(|err| err.relative_to(closing_paren_index + 1))?;

    let body = parse_stmt_list(&tokens[closing_paren_index + 2..][..closing_brace_index - 1])
        .map_err(|err| err.relative_to(closing_paren_index + 1))?;

    Ok(FoundStmt {
        stmt: Stmt::WhileLoop(ast::WhileLoop {
            condition: expression,
            body,
        }),
        next_index: closing_paren_index + 1 + closing_brace_index + 1,
    })
}

fn parse_expr_list(tokens: &[Token], seperator: ShallowTokenKind) -> Result<FoundExprList, Error> {
    let closing_paren_index = locate_last_matched_right(
        tokens,
        ShallowTokenKind::LeftParen,
        ShallowTokenKind::RightParen,
    )?;

    let mut items = Vec::new();

    let mut current = 1;

    while let Ok(sep_index) = tokens[current..closing_paren_index].locate_first(seperator) {
        dbg!(&tokens[current..closing_paren_index], sep_index);
        let FoundExpr { expression, .. } =
            parse_expr(&tokens[current..][..sep_index]).map_err(|err| err.relative_to(current))?;
        items.push(expression);
        current = current + sep_index + 1;
    }

    if let Ok(FoundExpr { expression, .. }) =
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
fn parse_prop_ident_list(tokens: &[Token]) -> Result<FoundPropIdentList, Error> {
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

/// Meant for brackets.
///
/// For example, finding the final `]` in `[a + [b]]`.
///
/// First token must be `left`
///
/// Returns an error for expected `right` if not found.
fn locate_last_matched_right(
    tokens: &[Token],
    left: ShallowTokenKind,
    right: ShallowTokenKind,
) -> Result<usize, Error> {
    tokens.get_token_kind(0, left)?;

    let mut left_parens = 1;

    for (index, token) in tokens.iter().enumerate().skip(1) {
        if token.kind.as_shallow() == left {
            left_parens += 1
        } else if token.kind.as_shallow() == right {
            if left_parens == 1 {
                return Ok(index);
            } else {
                left_parens -= 1;
            }
        }
    }

    Err(Error::expected_token(tokens.len() - 1, right, None))
}

#[cfg(test)]
mod tests {
    use ast::{FnDecl, Stmt};

    use super::{parse_fn_decl, parse_prop_ident_list, FoundStmt};
    use crate::lex::{lex_to_end, Token};
    use crate::parse::stmt_parsers::{parse_fn_call, parse_while_loop, parse_expr_list};

    fn tokenize(source: &str) -> Vec<Token> {
        let chars: Vec<char> = source.chars().collect();
        lex_to_end(&chars).expect("Failed to tokenize.")
    }

    #[test]
    fn parses_prop_list() {
        let tokens = tokenize("(a, b, c)");

        let props = parse_prop_ident_list(&tokens).unwrap();

        assert_eq!(props.prop_idents, vec!["a", "b", "c"])
    }

    #[test]
    fn parses_fn_decl() {
        let tokens = tokenize("fn main(a, b) { let cat = 2 + 3 / 2; }");

        let res = parse_fn_decl(&tokens);

        dbg!(res.unwrap());
    }

    #[test]
    fn parses_fn_call() {
        let tokens = tokenize("test(a + 12, b);");

        let res = parse_fn_call(&tokens);

        dbg!(res.unwrap());
    }

    #[test]
    fn parses_while_loop() {
        let tokens = tokenize("while (true){ test(); }");

        let res = parse_while_loop(&tokens);

        dbg!(res.unwrap());
    }

    #[test]
    fn parses_empty_expr_list(){
        let tokens = tokenize("()");

        dbg!(tokens
            .iter()
            .map(|t| t.kind.as_shallow())
            .collect::<Vec<_>>());

        let res = parse_expr_list(&tokens, crate::lex::ShallowTokenKind::Comma);

        dbg!(res.unwrap());
    }
}
