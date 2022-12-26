use is_macro::Is;

use crate::{AssignOpKind, Expr};

#[derive(Debug, Is, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    VarAssign(VarAssign),
    FnDecl(FnDecl),
    WhileLoop(WhileLoop),
    FnReturn(FnReturn),
    IfElse(IfElse),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ident: String,
    pub initializer: Expr,
}

#[derive(Debug, Clone)]
pub struct VarAssign {
    pub to: Expr,
    pub value: Expr,
    pub op: AssignOpKind,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub ident: String,
    pub prop_idents: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct FnReturn {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfElse {
    pub condition: Expr,
    pub true_branch: Vec<Stmt>,
    pub else_branch: Vec<Stmt>,
}
