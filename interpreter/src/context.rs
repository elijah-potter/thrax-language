use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
struct Scope {
    vars: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Context {
    scope: Vec<Scope>,
}
