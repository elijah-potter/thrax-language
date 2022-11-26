use ast::{FnDecl, FnReturn, Stmt, VarAssign, VarDecl};

use super::common_parsers::{parse_prop_ident_list, FoundPropIdentList};
use super::expr_parsers::parse_expr;
use super::tokens_ext::TokensExt;
use super::Error;
use crate::lex::{ShallowTokenKind, Token};

#[derive(Debug, Clone)]
pub struct FoundStmt {
    pub stmt: Stmt,
    pub next_index: usize,
}

pub fn parse_stmt_list(tokens: &[Token]) -> Result<Vec<Stmt>, Error> {
    let mut stmts = Vec::new();

    let mut current_index = 0;

    while current_index < tokens.len() {
        let FoundStmt { stmt, next_index } =
            parse_stmt(&tokens[current_index..]).map_err(|err| err.relative_to(current_index))?;

        current_index += next_index;
        stmts.push(stmt);
    }

    Ok(stmts)
}

/// Runs all parsers over supplied source, returning the first success or last failure
pub fn parse_stmt(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let parsers = [
        parse_var_decl,
        parse_var_assign,
        parse_fn_decl,
        parse_while_loop,
        parse_if_else,
        parse_return,
        parse_expr_stmt,
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

fn parse_var_decl(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::Let)?;

    let identifier = tokens.get_token_kind(1, ShallowTokenKind::Ident)?;

    tokens.get_token_kind(2, ShallowTokenKind::Equals)?;

    let semi_location = tokens.locate_first(0, ShallowTokenKind::Semicolon)?;

    let expr = parse_expr(&tokens[3..semi_location]).map_err(|err| err.relative_to(3))?;

    Ok(FoundStmt {
        stmt: Stmt::VarDecl(VarDecl {
            ident: identifier.clone().ident().unwrap(),
            initializer: expr,
        }),
        next_index: semi_location + 1,
    })
}

fn parse_var_assign(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let identifier = tokens.get_token_kind(0, ShallowTokenKind::Ident)?;

    tokens.get_token_kind(1, ShallowTokenKind::Equals)?;

    let semi_location = tokens.locate_first(0, ShallowTokenKind::Semicolon)?;

    let expr = parse_expr(&tokens[2..semi_location]).map_err(|err| err.relative_to(3))?;

    Ok(FoundStmt {
        stmt: Stmt::VarAssign(VarAssign {
            ident: identifier.clone().ident().unwrap(),
            value: expr,
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

    let FoundBody {
        body,
        next_index: after_body,
    } = parse_body(&tokens[next_index + 2..]).map_err(|err| err.relative_to(next_index + 2))?;

    Ok(FoundStmt {
        stmt: Stmt::FnDecl(FnDecl {
            ident: identifier.clone().ident().unwrap(),
            prop_idents,
            body,
        }),
        next_index: next_index + 2 + after_body,
    })
}

fn parse_while_loop(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::While)?;

    let closing_paren_index = (&tokens[1..])
        .locate_last_matched_right(ShallowTokenKind::LeftParen, ShallowTokenKind::RightParen)
        .map_err(|err| err.relative_to(1))?
        + 1;

    let expr = parse_expr(&tokens[2..closing_paren_index]).map_err(|err| err.relative_to(2))?;

    let FoundBody {
        body,
        next_index: after_body,
    } = parse_body(&tokens[closing_paren_index + 1..])
        .map_err(|err| err.relative_to(closing_paren_index + 1))?;

    Ok(FoundStmt {
        stmt: Stmt::WhileLoop(ast::WhileLoop {
            condition: expr,
            body,
        }),
        next_index: closing_paren_index + 1 + after_body,
    })
}

fn parse_return(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::Return)?;

    let final_semi = tokens.locate_first(0, ShallowTokenKind::Semicolon)?;

    let expr = if final_semi == 1 {
        None
    } else {
        Some(
            super::expr_parsers::parse_expr(&tokens[1..final_semi])
                .map_err(|err| err.relative_to(1))?,
        )
    };

    Ok(FoundStmt {
        stmt: Stmt::FnReturn(FnReturn { value: expr }),
        next_index: final_semi + 1,
    })
}

fn parse_if_else(tokens: &[Token]) -> Result<FoundStmt, Error> {
    tokens.get_token_kind(0, ShallowTokenKind::If)?;

    let closing_paren_index = (&tokens[1..])
        .locate_last_matched_right(ShallowTokenKind::LeftParen, ShallowTokenKind::RightParen)
        .map_err(|err| err.relative_to(1))?
        + 1;

    let condition =
        parse_expr(&tokens[2..closing_paren_index]).map_err(|err| err.relative_to(2))?;

    let FoundBody {
        body: true_branch,
        next_index: after_body,
    } = parse_body(&tokens[closing_paren_index + 1..])
        .map_err(|err| err.relative_to(closing_paren_index + 1))?;

    let after_body = after_body + closing_paren_index + 1;

    if tokens
        .get_token_kind(after_body, ShallowTokenKind::Else)
        .is_ok()
    {
        // Check if it is an `else if` statement
        if let Ok(FoundStmt {
            stmt,
            next_index: after_second_body,
        }) = parse_if_else(&tokens[after_body + 1..])
        {
            Ok(FoundStmt {
                stmt: Stmt::IfElse(ast::IfElse {
                    condition,
                    true_branch,
                    else_branch: vec![stmt],
                }),
                next_index: after_body + 1 + after_second_body,
            })
        }
        // Otherwise it's just an `else` statement
        else {
            let FoundBody {
                body: else_branch,
                next_index: after_second_body,
            } = parse_body(&tokens[after_body + 1..])
                .map_err(|err| err.relative_to(after_body + 1))?;
            Ok(FoundStmt {
                stmt: Stmt::IfElse(ast::IfElse {
                    condition,
                    true_branch,
                    else_branch,
                }),
                next_index: after_body + 1 + after_second_body,
            })
        }
    } else {
        Ok(FoundStmt {
            stmt: Stmt::IfElse(ast::IfElse {
                condition,
                true_branch,
                else_branch: Vec::new(),
            }),
            next_index: after_body,
        })
    }
}

fn parse_expr_stmt(tokens: &[Token]) -> Result<FoundStmt, Error> {
    let final_semi = tokens.locate_first(0, ShallowTokenKind::Semicolon)?;

    let expr = super::expr_parsers::parse_expr(&tokens[..final_semi])?;

    Ok(FoundStmt {
        stmt: Stmt::Expr(expr),
        next_index: final_semi + 1,
    })
}

struct FoundBody {
    body: Vec<Stmt>,
    next_index: usize,
}

fn parse_body(tokens: &[Token]) -> Result<FoundBody, Error> {
    let closing_brace_index = tokens
        .locate_last_matched_right(ShallowTokenKind::LeftBrace, ShallowTokenKind::RightBrace)?;

    let body =
        parse_stmt_list(&tokens[1..closing_brace_index]).map_err(|err| err.relative_to(1))?;

    Ok(FoundBody {
        body,
        next_index: closing_brace_index + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_fn_decl, parse_if_else};
    use crate::parse::stmt_parsers::parse_while_loop;
    use crate::test_utils::tokenize;

    #[test]
    fn parses_fn_decl() {
        let tokens = tokenize("fn main(a, b) { let cat = 2 + 3 / 2; }");

        let res = parse_fn_decl(&tokens);

        res.unwrap();
    }

    #[test]
    fn parses_while_loop() {
        let tokens = tokenize("while (true){ test(); }");

        let res = parse_while_loop(&tokens);

        res.unwrap();
    }

    #[test]
    fn parses_if() {
        let tokens = tokenize("if (true){ test(); }");

        let res = parse_if_else(&tokens);

        res.unwrap();
    }
}
