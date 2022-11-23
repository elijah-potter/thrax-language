use ast::Stmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Fn(Fn),
}

#[derive(Debug, Clone)]
pub struct Fn {
    prop_idents: Vec<String>,
    body: Vec<Stmt>,
}
