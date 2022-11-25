use std::collections::HashMap;

use ast::{BinaryOp, Expr, FnCall, FnDecl, Program, Stmt, VarAssign, VarDecl, WhileLoop};

use crate::error::Error;
use crate::value::{Fn, ShallowValue, Value};

type Scope = HashMap<String, Value>;

#[derive(Clone)]
pub enum Returnable {
    Returned(Option<Value>),
    Completed,
}

#[derive(Clone)]
pub struct Context {
    scopes: Vec<Scope>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add_native_function(&mut self, name: String, native_fn: fn(&[Value]) -> Result<Value, Error>){
        self.current_scope().insert(name, Value::Fn(Fn::Native(native_fn)));
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<Returnable, Error> {
        for stmt in program {
            if let Returnable::Returned(r) = self.eval_stmt(&stmt)? {
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

                let returned_value = self.eval_expr(&value)?;
                Ok(Returnable::Returned(Some(returned_value)))
            }
        }
    }

    fn eval_var_decl(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
        let Err(_) = self.value_of_ident(&var_decl.ident) else {
            return Err(Error::Redeclaration(var_decl.ident.clone()));
        };

        let initialized = self.eval_expr(&var_decl.initializer)?;

        self.current_scope()
            .insert(var_decl.ident.clone(), initialized);

        Ok(())
    }

    fn eval_var_assign(&mut self, var_assign: &VarAssign) -> Result<(), Error> {
        let new_value = self.eval_expr(&var_assign.value)?;

        let Ok(value) = self.value_of_ident(&var_assign.ident) else{
            return Err(Error::Undeclared(var_assign.ident.clone()));
        };

        *value = new_value;

        Ok(())
    }

    fn eval_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<(), Error> {
        let Err(_) = self.value_of_ident(&fn_decl.ident) else {
            return Err(Error::Redeclaration(fn_decl.ident.clone()));
        };

        self.current_scope().insert(
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
            self.scopes.push(Scope::new());

            let res = self.eval_program(&while_loop.body)?;

            self.scopes.pop();

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

        self.scopes.push(Scope::new());

        let res = self.eval_program(branch);

        self.scopes.pop();

        res
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Error> {
        match expr {
            Expr::Ident(i) => self.value_of_ident(i.as_str()).cloned(),
            Expr::NumberLiteral(n) => Ok(Value::Number(*n)),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone())),
            Expr::BoolLiteral(b) => Ok(Value::Bool(*b)),
            Expr::ArrayLiteral(arr) => {
                let mut results = Vec::with_capacity(arr.len());
                for expr in arr.into_iter() {
                    let result = self.eval_expr(expr)?;
                    results.push(result);
                }
                Ok(Value::Array(results))
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
                    let result = self.eval_expr(&arg)?;
                    args.push(result);
                }

                // There's got to be a way around this clone
                let definition = self.value_of_ident(&f.ident)?.clone();

                let Value::Fn(df) = definition else{
                    return Err(Error::TypeError(ShallowValue::Fn, definition.as_shallow()));
                };

                match df {
                    Fn::Native(native_fn) => native_fn(&args),
                    Fn::Interpreted { prop_idents, body } => {
                        let scope = self.scope_of_ident(&f.ident)?;
                        let mut old = if scope == self.scopes.len() - 1 {
                            Vec::new()
                        } else {
                            self.scopes.split_off(scope + 1)
                        };

                        // Build current scope with arguments
                        let mut new_scope = Scope::with_capacity(args.len());
                        for (ident, arg) in prop_idents.iter().zip(args.iter()) {
                            new_scope.insert(ident.clone(), arg.clone());
                        }
                        self.scopes.push(new_scope);

                        let res = self.eval_program(&body)?;

                        self.scopes.pop();
                        // Replace old scopes
                        self.scopes.append(&mut old);

                        if let Returnable::Returned(r) = res {
                            return Ok(r.unwrap_or(Value::Null));
                        }

                        Ok(Value::Null)
                    }
                }
            }
        }
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn value_of_ident<'a>(&'a mut self, ident: &str) -> Result<&'a mut Value, Error> {
        self.scopes
            .iter_mut()
            .rev()
            .find_map(|scope| scope.get_mut(ident))
            .ok_or(Error::UndefinedAccess(ident.to_string()))
    }

    fn scope_of_ident(&self, ident: &str) -> Result<usize, Error> {
        self.scopes
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, scope)| scope.get(ident).map(|_| index))
            .ok_or(Error::UndefinedAccess(ident.to_string()))
    }
}
