use std::collections::HashMap;

use is_macro::Is;

#[derive(Debug, Is, Clone)]
pub enum Expr {
    Ident(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    ArrayLiteral(Vec<Expr>),
    ObjectLiteral(HashMap<String, Expr>),
    BinaryOp(BinaryOp),
    FnCall(FnCall),
    Member(Member),
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub kind: BinaryOpKind,
    pub a: Box<Expr>,
    pub b: Box<Expr>,
}

#[derive(Debug, Is, Clone, Copy)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equals,
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub ident: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub parent: Box<Expr>,
    pub child: Box<Expr>,
}
