use is_macro::Is;

#[derive(Debug, Is, Clone)]
pub enum Expr {
    Ident(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    BinaryOp(BinaryOp),
    FnCall(FnCall),
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
