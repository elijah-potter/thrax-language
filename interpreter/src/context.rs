use std::collections::{HashSet, HashMap};

use ast::{BinaryOp, Expr, FnDecl, Program, Stmt, VarAssign, VarDecl, WhileLoop};

use crate::error::Error;
use crate::heap::{Heap, HeapItem};
use crate::stack::{FoundIdent, Stack};
use crate::stdlib::add_stdlib;
use crate::value::{Fn, ShallowValue, Value};

#[derive(Clone)]
pub enum Returnable {
    Returned(Option<Value>),
    Completed,
}

#[derive(Clone)]
pub struct Context {
    stack: Stack,
    arrays: Heap<Vec<Value>>,
    objects: Heap<HashMap<String, Value>>,
    use_gc: bool,
}

impl Context {
    #[must_use] pub fn new(use_gc: bool) -> Self {
        Self {
            stack: Stack::new(),
            arrays: Heap::new(),
            objects: Heap::new(),
            use_gc,
        }
    }

    pub fn add_native_function(
        &mut self,
        name: String,
        native_fn: fn(&mut Self, &[Value]) -> Result<Value, Error>,
    ) {
        self.stack
            .push_value(name, Value::Fn(Fn::Native(native_fn)));
    }

    /// Courtesey wrapper for [`crate::stdlib::add_stdlib`]
    pub fn add_stdlib(&mut self) {
        add_stdlib(self)
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<Returnable, Error> {
        for stmt in program {
            if let Returnable::Returned(r) = self.eval_stmt(stmt)? {
                return Ok(Returnable::Returned(r));
            }
        }

        Ok(Returnable::Completed)
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Returnable, Error> {
        match stmt {
            Stmt::VarDecl(var_decl) => self.eval_var_decl(var_decl).map(|_| Returnable::Completed),
            Stmt::VarAssign(var_assign) => self
                .eval_var_assign(var_assign)
                .map(|_| Returnable::Completed),
            Stmt::FnDecl(fn_decl) => self.eval_fn_decl(fn_decl).map(|_| Returnable::Completed),
            Stmt::Expr(expr) => self
                .eval_expr(expr)
                .map(|_| ())
                .map(|_| Returnable::Completed),
            Stmt::IfElse(if_else) => self.eval_if_else(if_else),
            Stmt::WhileLoop(while_loop) => self.eval_while_loop(while_loop),
            Stmt::FnReturn(fn_return) => {
                let Some(value) = &fn_return.value else{
                    return Ok(Returnable::Returned(None));
                };

                let returned_value = self.eval_expr(value)?;
                Ok(Returnable::Returned(Some(returned_value)))
            }
        }
    }

    fn eval_var_decl(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
        let Err(_) = self.find_with_ident(&var_decl.ident) else {
            return Err(Error::Redeclaration(var_decl.ident.clone()));
        };

        let initialized = self.eval_expr(&var_decl.initializer)?;

        self.stack.push_value(var_decl.ident.clone(), initialized);

        Ok(())
    }

    fn eval_var_assign(&mut self, var_assign: &VarAssign) -> Result<(), Error> {
        let new_value = self.eval_expr(&var_assign.value)?;

        let Ok(FoundIdent { value, .. }) = self.find_with_ident(&var_assign.ident) else{
            return Err(Error::Undeclared(var_assign.ident.clone()));
        };

        *value = new_value;

        Ok(())
    }

    fn eval_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<(), Error> {
        let Err(_) = self.find_with_ident(&fn_decl.ident) else {
            return Err(Error::Redeclaration(fn_decl.ident.clone()));
        };

        self.stack.push_value(
            fn_decl.ident.clone(),
            Value::Fn(Fn::Interpreted {
                prop_idents: fn_decl.prop_idents.clone(),
                body: fn_decl.body.clone(),
            }),
        );

        Ok(())
    }

    fn eval_while_loop(&mut self, while_loop: &WhileLoop) -> Result<Returnable, Error> {
        while let Value::Bool(true) = self
            .eval_expr(&while_loop.condition)?
            .equals(&Value::Bool(true))?
        {
            self.stack.open_frame();

            let res = self.eval_program(&while_loop.body)?;

            self.stack.pop_frame();

            if let Returnable::Returned(r) = res {
                return Ok(Returnable::Returned(r));
            }
        }

        Ok(Returnable::Completed)
    }

    fn eval_if_else(&mut self, if_else: &ast::IfElse) -> Result<Returnable, Error> {
        let branch = match self
            .eval_expr(&if_else.condition)?
            .equals(&Value::Bool(true))?
        {
            Value::Bool(true) => &if_else.true_branch,
            Value::Bool(false) => &if_else.else_branch,
            _ => panic!(),
        };

        self.stack.open_frame();

        let res = self.eval_program(branch);

        self.stack.pop_frame();

        res
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Ident(i) => self.find_with_ident(i).map(|v| v.value.clone()),
            Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::BoolLiteral(b) => Ok(Value::Bool(*b)),
            Expr::ArrayLiteral(arr) => {
                if self.use_gc {
                    self.collect_garbage();
                }

                let mut results = Vec::with_capacity(arr.len());
                for expr in arr.iter() {
                    let result = self.eval_expr(expr)?;
                    results.push(result);
                }
                Ok(Value::Array(self.arrays.push(results)))
            },
            Expr::ObjectLiteral(obj) => {
                if self.use_gc {
                    self.collect_garbage();
                }

                let mut results = HashMap::with_capacity(obj.len());

                for (key, expr) in obj{
                    let result = self.eval_expr(expr)?;
                    results.insert(key.to_string(), result);
                }

                Ok(Value::Object(self.objects.push(results)))
            }
            Expr::BinaryOp(BinaryOp { kind, a, b }) => {
                let c_a = self.eval_expr(a)?;
                let c_b = self.eval_expr(b)?;

                match kind {
                    ast::BinaryOpKind::Add => c_a.add(&c_b),
                    ast::BinaryOpKind::Subtract => c_a.subtract(&c_b),
                    ast::BinaryOpKind::Multiply => c_a.multiply(&c_b),
                    ast::BinaryOpKind::Divide => c_a.divide(&c_b),
                    ast::BinaryOpKind::GreaterThan => c_a.greater_than(&c_b),
                    ast::BinaryOpKind::LessThan => c_a.less_than(&c_b),
                    ast::BinaryOpKind::Equals => c_a.equals(&c_b),
                }
            }
            Expr::FnCall(f) => {
                let mut args = Vec::with_capacity(f.args.len());

                for arg in &f.args {
                    let result = self.eval_expr(arg)?;
                    args.push(result);
                }

                // There's got to be a way around this clone
                let FoundIdent {
                    value: definition,
                    index: definition_index,
                } = self.find_with_ident(&f.ident)?;

                let Value::Fn(df) = definition.clone() else{
                    return Err(Error::TypeError(ShallowValue::Fn, definition.as_shallow()));
                };

                match df {
                    Fn::Native(native_fn) => native_fn(self, &args),
                    Fn::Interpreted { prop_idents, body } => {
                        let old = self.stack.pop_until_index(definition_index);

                        let mut new_scope = Vec::with_capacity(args.len());

                        if args.len() != prop_idents.len() {
                            return Err(Error::IncorrectArgumentCount(
                                prop_idents.len(),
                                args.len(),
                            ));
                        }

                        for (ident, value) in prop_idents.iter().zip(args.iter()) {
                            new_scope.push((ident.clone(), value.clone()));
                        }
                        self.stack.push_frame(new_scope);

                        let res = self.eval_program(&body)?;

                        self.stack.pop_frame();
                        // Replace old scopes
                        self.stack.push_popped_stack(old);

                        if let Returnable::Returned(r) = res {
                            return Ok(r.unwrap_or(Value::Null));
                        }

                        Ok(Value::Null)
                    }
                }
            }
        }
    }

    pub fn find_with_ident<'a>(&'a mut self, ident: &str) -> Result<FoundIdent<'a>, Error> {
        self.stack
            .find_with_ident(ident)
            .ok_or(Error::Undeclared(ident.to_string()))
    }

    pub fn get_array_mut<'a>(&'a mut self, key: &HeapItem<Vec<Value>>) -> &'a mut Vec<Value> {
        self.arrays
            .get_mut(key)
    }

    pub fn get_array<'a>(&'a self, key: &HeapItem<Vec<Value>>) -> &'a Vec<Value> {
        self.arrays.get(key)
    }

    pub fn get_object_mut<'a>(&'a mut self, key: &HeapItem<HashMap<String, Value>>) -> &'a mut HashMap<String, Value> {
        self.objects
            .get_mut(key)
    }

    pub fn get_object<'a>(&'a self, key: &HeapItem<HashMap<String, Value>>) -> &'a HashMap<String, Value> {
        self.objects.get(key)
    }

    pub fn collect_garbage(&mut self) {
        let mut to_search = Vec::new();

        for item in self.stack.iter_values() {
            match item {
                Value::Array(arr_id) => {
                    to_search.push(*arr_id);
                }
                _ => (),
            }
        }

        let mut visited = HashSet::new();

        while let Some(arr_id) = to_search.pop() {
            if visited.contains(&arr_id) {
                continue;
            }

            let arr = self.get_array(&arr_id);

            visited.insert(arr_id);

            for item in arr {
                match item {
                    Value::Array(arr_id) => {
                        to_search.push(*arr_id);
                    }
                    _ => (),
                }
            }
        }

        let visited: Vec<_> = visited.into_iter().collect();

        self.arrays.filter_keys(visited.as_slice());
    }

    /// Get the number of [Value]'s in the stack
    pub fn stack_size(&self) -> usize {
        self.stack.value_len()
    }

    /// Get the number of arrays in the array heap
    pub fn array_heap_size(&self) -> usize {
        self.arrays.len()
    }
}
