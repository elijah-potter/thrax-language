use is_macro::Is;

use crate::Expr;

#[derive(Debug, Is, Clone)]
pub enum Stmt {
    VarDecl(VarDecl),
    VarAssign(VarAssign),
    FnDecl(FnDecl),
    FnCall(FnCall),
    WhileLoop(WhileLoop),
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub ident: String,
    pub initializer: Expr,
}

#[derive(Debug, Clone)]
pub struct VarAssign {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub ident: String,
    pub prop_idents: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub ident: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: Expr,
    pub body: Vec<Stmt>,
}
