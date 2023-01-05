use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use ast::{
    AssignOpKind, BinaryOp, Expr, FnCall, FnDecl, Member, Program, Stmt, VarAssign, VarDecl,
    WhileLoop,
};

use crate::error::Error;
use crate::stack::{FoundIdent, Stack};
use crate::stdlib::add_stdlib;
use crate::value::{Fn, GcValue, NativeFn, ShallowValue, Value};

#[derive(Clone)]
pub enum BlockExit {
    Returned(Option<GcValue>),
    Break,
    Continue,
    Completed,
}

#[derive(Clone)]
pub struct Context {
    stack: Stack<GcValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    pub fn add_native_function(&mut self, name: String, native_fn: NativeFn) {
        self.stack
            .push_value(name, Value::Fn(Rc::new(Fn::Native(native_fn))).into_gc());
    }

    /// Courtesey wrapper for [`crate::stdlib::add_stdlib`]
    pub fn add_stdlib(&mut self) {
        add_stdlib(self)
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<BlockExit, Error> {
        for stmt in program {
            let res = self.eval_stmt(stmt)?;

            if !matches!(res, BlockExit::Completed) {
                return Ok(res);
            }
        }

        Ok(BlockExit::Completed)
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> Result<BlockExit, Error> {
        match stmt {
            Stmt::VarDecl(var_decl) => self.eval_var_decl(var_decl).map(|_| BlockExit::Completed),
            Stmt::VarAssign(var_assign) => self
                .eval_var_assign(var_assign)
                .map(|_| BlockExit::Completed),
            Stmt::FnDecl(fn_decl) => self.eval_fn_decl(fn_decl).map(|_| BlockExit::Completed),
            Stmt::Expr(expr) => self
                .eval_expr(expr)
                .map(|_| ())
                .map(|_| BlockExit::Completed),
            Stmt::IfElse(if_else) => self.eval_if_else(if_else),
            Stmt::WhileLoop(while_loop) => self.eval_while_loop(while_loop),
            Stmt::BlockExit(block_exit) => {
                let exit = match block_exit {
                    ast::BlockExit::FnReturn(res) => {
                        if let Some(expr) = res {
                            BlockExit::Returned(Some(self.eval_expr(expr)?))
                        } else {
                            BlockExit::Returned(None)
                        }
                    }
                    ast::BlockExit::Break => BlockExit::Break,
                    ast::BlockExit::Continue => BlockExit::Continue,
                };

                Ok(exit)
            }
        }
    }

    fn eval_var_decl(&mut self, var_decl: &VarDecl) -> Result<(), Error> {
        let Err(_) = self.find_with_ident(&var_decl.ident) else {
            return Err(Error::Redeclaration(var_decl.ident.clone()));
        };

        let initialized = self.eval_expr(&var_decl.initializer)?.shallow_copy();

        self.stack.push_value(var_decl.ident.clone(), initialized);

        Ok(())
    }

    fn eval_var_assign(&mut self, var_assign: &VarAssign) -> Result<(), Error> {
        let new_value = self.eval_expr(&var_assign.value)?.shallow_copy();
        let new_value = new_value.borrow();

        let value = self.eval_expr(&var_assign.to)?;
        let mut value = value.borrow_mut();

        match var_assign.op {
            AssignOpKind::NoOp => {
                *value = new_value.clone();
            }
            AssignOpKind::Op(op) => {
                let arith_res = value.run_binary_op(&new_value, op)?;

                *value = arith_res;
            }
        }

        Ok(())
    }

    fn eval_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<(), Error> {
        let Err(_) = self.find_with_ident(&fn_decl.ident) else {
            return Err(Error::Redeclaration(fn_decl.ident.clone()));
        };

        self.stack.push_value(
            fn_decl.ident.clone(),
            Value::Fn(Rc::new(Fn::Interpreted {
                prop_idents: fn_decl.prop_idents.clone(),
                body: fn_decl.body.clone(),
            }))
            .into_gc(),
        );

        Ok(())
    }

    fn eval_while_loop(&mut self, while_loop: &WhileLoop) -> Result<BlockExit, Error> {
        while let Value::Bool(true) = {
            let res = self.eval_expr(&while_loop.condition)?;
            let res = res.borrow();
            res.equals(&Value::Bool(true))?
        } {
            self.stack.open_frame();

            let res = self.eval_program(&while_loop.body)?;

            self.stack.pop_frame();

            match res {
                BlockExit::Returned(r) => return Ok(BlockExit::Returned(r)),
                BlockExit::Break => return Ok(BlockExit::Completed),
                _ => (),
            }
        }

        Ok(BlockExit::Completed)
    }

    fn eval_if_else(&mut self, if_else: &ast::IfElse) -> Result<BlockExit, Error> {
        let branch = match {
            let res = self.eval_expr(&if_else.condition)?;
            let res = res.borrow();
            res.equals(&Value::Bool(true))?
        } {
            Value::Bool(true) => &if_else.true_branch,
            Value::Bool(false) => &if_else.else_branch,
            _ => panic!(),
        };

        self.stack.open_frame();

        let res = self.eval_program(branch);

        self.stack.pop_frame();

        res
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<GcValue, Error> {
        match expr {
            Expr::Ident(i) => self.find_with_ident(i).map(|v| v.value),
            Expr::NumberLiteral(n) => Ok(Value::Number(*n).into_gc()),
            Expr::StringLiteral(s) => Ok(Value::String(s.clone()).into_gc()),
            Expr::BoolLiteral(b) => Ok(Value::Bool(*b).into_gc()),
            Expr::ArrayLiteral(arr) => self.eval_array_lit(arr),
            Expr::ObjectLiteral(obj) => self.eval_object_lit(obj),
            Expr::BinaryOp(bin_op) => self.eval_binary_op(bin_op),
            Expr::FnCall(f) => self.run_fn(f),
            Expr::Member(m) => self.eval_member(m),
        }
    }

    fn eval_member(&mut self, member: &Member) -> Result<GcValue, Error> {
        let child = self.eval_expr(&member.child)?;
        let child = child.borrow();

        let parent = self.eval_expr(&member.parent)?;
        let parent = parent.borrow_mut();

        match &*parent {
            Value::String(s) => {
                if let Value::Number(index) = *child {
                    let rounded = index.floor();
                    if rounded == index {
                        s.chars()
                            .nth(rounded as usize)
                            .ok_or(Error::IndexOutOfBounds(rounded as usize))
                            .map(|c| Value::String(c.to_string()).into_gc()) // TODO: Once we add chars, remove this last bit
                    } else {
                        Err(Error::ExpectedInteger(index))
                    }
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            Value::Array(arr) => {
                if let Value::Number(index) = *child {
                    let rounded = index.floor();

                    if rounded == index {
                        arr.get(rounded as usize)
                            .cloned()
                            .ok_or(Error::IndexOutOfBounds(rounded as usize))
                    } else {
                        Err(Error::ExpectedInteger(index))
                    }
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            Value::Object(obj) => {
                if let Value::String(index) = &*child {
                    obj.get(index)
                        .cloned()
                        .ok_or_else(|| Error::ObjectMissingKey(index.clone()))
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            _ => Err(Error::CannotIndexType(parent.as_shallow())),
        }
    }

    fn eval_array_lit(&mut self, arr: &[Expr]) -> Result<GcValue, Error> {
        let mut results = VecDeque::with_capacity(arr.len());
        for expr in arr.iter() {
            let result = self.eval_expr(expr)?;
            results.push_back(result);
        }
        Ok((Value::Array(results)).into_gc())
    }

    fn eval_object_lit(&mut self, obj: &HashMap<String, Expr>) -> Result<GcValue, Error> {
        let mut results = HashMap::with_capacity(obj.len());

        for (key, expr) in obj {
            let result = self.eval_expr(expr)?;
            results.insert(key.to_string(), result);
        }

        Ok(Value::Object(results).into_gc())
    }

    fn eval_binary_op(&mut self, bin_op: &BinaryOp) -> Result<GcValue, Error> {
        let BinaryOp { kind, a, b } = bin_op;

        let c_a = self.eval_expr(a)?;
        let c_a = c_a.borrow();
        let c_b = self.eval_expr(b)?;
        let c_b = c_b.borrow();

        let arith_res = c_a.run_binary_op(&c_b, *kind)?;

        Ok((arith_res).into_gc())
    }

    pub fn run_fn(&mut self, fn_call: &FnCall) -> Result<GcValue, Error> {
        let mut args = Vec::with_capacity(fn_call.args.len());

        for arg in &fn_call.args {
            let result = self.eval_expr(arg)?;
            args.push(result.shallow_copy());
        }

        let (fn_def, definition_index) = {
            let FoundIdent {
                value: definition,
                index: definition_index,
            } = self.find_with_ident(&fn_call.ident)?;

            let definition = definition.borrow();

            let Value::Fn(df) = &*definition else{
                        return Err(Error::TypeError(ShallowValue::Fn, definition.as_shallow()));
                    };

            (df.clone(), definition_index)
        };

        match fn_def.as_ref() {
            Fn::Native(native_fn) => native_fn(self, &args),
            Fn::Interpreted { prop_idents, body } => {
                let old = self.stack.pop_until_index(definition_index);

                let mut new_scope = Vec::with_capacity(args.len());

                if args.len() != prop_idents.len() {
                    return Err(Error::IncorrectArgumentCount(prop_idents.len(), args.len()));
                }

                for (ident, value) in prop_idents.iter().zip(args.iter()) {
                    new_scope.push((ident.clone(), value.clone()));
                }
                self.stack.push_frame(new_scope);

                let res = self.eval_program(body)?;

                self.stack.pop_frame();
                // Replace old scopes
                self.stack.push_popped_stack(old);

                if let BlockExit::Returned(r) = res {
                    return Ok(r.unwrap_or_else(|| (Value::Null).into_gc()));
                }

                Ok((Value::Null).into_gc())
            }
        }
    }

    pub fn find_with_ident(&self, ident: &str) -> Result<FoundIdent<GcValue>, Error> {
        self.stack
            .find_with_ident(ident)
            .ok_or_else(|| Error::Undeclared(ident.to_string()))
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
