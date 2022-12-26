use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use ast::{BinaryOp, Expr, FnCall, FnDecl, Member, Program, Stmt, VarAssign, VarDecl, WhileLoop};

use crate::error::Error;
use crate::heap::{Heap, HeapItem};
use crate::stack::{FoundIdent, FoundIdentMut, Stack};
use crate::stdlib::add_stdlib;
use crate::value::{Fn, ShallowValue, Value};

#[derive(Clone)]
pub enum Returnable {
    Returned(Option<HeapItem<Value>>),
    Completed,
}

type ValueRef = HeapItem<Value>;

#[derive(Clone)]
pub struct Context {
    /// Values on the stack refer to values in `values`
    stack: Stack<ValueRef>,
    pub(crate) values: Heap<Value>,
    pub(crate) arrays: Heap<Vec<HeapItem<Value>>>,
    pub(crate) objects: Heap<HashMap<String, HeapItem<Value>>>,
    use_gc: bool,
}

impl Context {
    #[must_use]
    pub fn new(use_gc: bool) -> Self {
        Self {
            stack: Stack::new(),
            values: Heap::new(),
            arrays: Heap::new(),
            objects: Heap::new(),
            use_gc,
        }
    }

    pub fn add_native_function(
        &mut self,
        name: String,
        native_fn: fn(&mut Self, &[HeapItem<Value>]) -> Result<HeapItem<Value>, Error>,
    ) {
        self.stack.push_value(
            name,
            self.values.push(Value::Fn(Rc::new(Fn::Native(native_fn)))),
        );
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
        if self.use_gc {
            self.collect_garbage();
        }

        let Err(_) = self.find_with_ident(&var_decl.ident) else {
            return Err(Error::Redeclaration(var_decl.ident.clone()));
        };

        let initialized = self.eval_expr(&var_decl.initializer)?;

        self.stack.push_value(var_decl.ident.clone(), initialized);

        Ok(())
    }

    fn eval_var_assign(&mut self, var_assign: &VarAssign) -> Result<(), Error> {
        let value = self.eval_expr(&var_assign.to)?;
        let new_value = self.eval_expr(&var_assign.value)?;


        match var_assign.op{
            ast::AssignOpKind::NoOp => value.set(new_value.get_inner().clone()),
            ast::AssignOpKind::Op(op) => {
               let arith_res = value.get_inner().run_binary_op(new_value.get_inner(), op)?;

               value.set(arith_res)
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
            self.values.push(Value::Fn(Rc::new(Fn::Interpreted {
                prop_idents: fn_decl.prop_idents.clone(),
                body: fn_decl.body.clone(),
            }))),
        );

        Ok(())
    }

    fn eval_while_loop(&mut self, while_loop: &WhileLoop) -> Result<Returnable, Error> {
        while let Value::Bool(true) = self
            .eval_expr(&while_loop.condition)?
            .get_inner()
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
            .get_inner()
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

    fn eval_expr(&mut self, expr: &Expr) -> Result<HeapItem<Value>, Error> {
        match expr {
            Expr::Ident(i) => self.find_with_ident(i).map(|v| v.value.clone()),
            Expr::NumberLiteral(n) => Ok(self.values.push(Value::Number(*n))),
            Expr::StringLiteral(s) => Ok(self.values.push(Value::String(s.clone()))),
            Expr::BoolLiteral(b) => Ok(self.values.push(Value::Bool(*b))),
            Expr::ArrayLiteral(arr) => self.eval_array_lit(arr),
            Expr::ObjectLiteral(obj) => self.eval_object_lit(obj),
            Expr::BinaryOp(bin_op) => self.eval_binary_op(bin_op),
            Expr::FnCall(f) => self.run_fn(f),
            Expr::Member(m) => self.eval_member(m),
        }
    }

    fn eval_member(&mut self, member: &Member) -> Result<HeapItem<Value>, Error> {
        let parent = self.eval_expr(&member.parent)?;
        let parent = parent.get_inner();
        let child = self.eval_expr(&member.child)?;
        let child = child.get_inner();

        match parent {
            Value::String(s) => {
                if let Value::Number(index) = child {
                    let rounded = index.floor();
                    if rounded == *index {
                        s.chars()
                            .nth(rounded as usize)
                            .ok_or(Error::IndexOutOfBounds(rounded as usize))
                            .map(|c| self.values.push(c.to_string().into())) // TODO: Once we add chars, remove this last bit
                    } else {
                        Err(Error::ExpectedInteger(*index))
                    }
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            Value::Array(arr_id) => {
                if let Value::Number(index) = child {
                    let arr = self.arrays.get(&arr_id);
                    let rounded = index.floor();

                    if rounded == *index {
                        arr.get(rounded as usize)
                            .cloned()
                            .ok_or(Error::IndexOutOfBounds(rounded as usize))
                    } else {
                        Err(Error::ExpectedInteger(*index))
                    }
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            Value::Object(obj_id) => {
                if let Value::String(index) = child {
                    let obj = self.objects.get(&obj_id);
                    obj.get(index)
                        .cloned()
                        .ok_or(Error::ObjectMissingKey(index.clone()))
                } else {
                    Err(Error::TypeError(ShallowValue::Number, child.as_shallow()))
                }
            }
            _ => Err(Error::CannotIndexType(parent.as_shallow())),
        }
    }

    fn eval_array_lit(&mut self, arr: &[Expr]) -> Result<HeapItem<Value>, Error> {
        let mut results = Vec::with_capacity(arr.len());
        for expr in arr.iter() {
            let result = self.eval_expr(expr)?;
            results.push(result);
        }
        Ok(self.values.push(Value::Array(self.arrays.push(results))))
    }

    fn eval_object_lit(&mut self, obj: &HashMap<String, Expr>) -> Result<HeapItem<Value>, Error> {
        let mut results = HashMap::with_capacity(obj.len());

        for (key, expr) in obj {
            let result = self.eval_expr(expr)?;
            results.insert(key.to_string(), result);
        }

        Ok(self.values.push(Value::Object(self.objects.push(results))))
    }

    fn eval_binary_op(&mut self, bin_op: &BinaryOp) -> Result<HeapItem<Value>, Error> {
        let BinaryOp { kind, a, b } = bin_op;

        let c_a = self.eval_expr(a)?;
        let c_a = c_a.get_inner();
        let c_b = self.eval_expr(b)?;
        let c_b = c_b.get_inner();

        let arith_res = c_a.run_binary_op(c_b, *kind)?; 

            Ok(self.values.push(arith_res))
    }

    pub fn run_fn(&mut self, fn_call: &FnCall) -> Result<HeapItem<Value>, Error> {
        let mut args = Vec::with_capacity(fn_call.args.len());

        for arg in &fn_call.args {
            let result = self.eval_expr(arg)?;
            args.push(result);
        }

        let (fn_def, definition_index) = {
            let FoundIdent {
                value: definition,
                index: definition_index,
            } = self.find_with_ident(&fn_call.ident)?;

            let definition = definition.get_inner();

            let Value::Fn(df) = definition else{
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

                if let Returnable::Returned(r) = res {
                    return Ok(r.unwrap_or(self.values.push(Value::Null)));
                }

                Ok(self.values.push(Value::Null))
            }
        }
    }

    pub fn find_with_ident_mut<'a>(
        &'a mut self,
        ident: &str,
    ) -> Result<FoundIdentMut<HeapItem<Value>>, Error> {
        self.stack
            .find_with_ident_mut(ident)
            .ok_or_else(|| Error::Undeclared(ident.to_string()))
    }

    pub fn find_with_ident<'a>(
        &'a self,
        ident: &str,
    ) -> Result<FoundIdent<HeapItem<Value>>, Error> {
        self.stack
            .find_with_ident(ident)
            .ok_or_else(|| Error::Undeclared(ident.to_string()))
    }

    pub fn collect_garbage(&mut self) {
        let mut search_queue: VecDeque<_> = self.stack.iter_values().collect();
        let mut completed = HashSet::new();
        let mut completed_arrays = HashSet::new();
        let mut completed_objects = HashSet::new();

        while let Some(value) = search_queue.pop_front() {
            if completed.contains(&value) {
                continue;
            }

            match value.get_inner() {
                Value::Array(arr_id) => {
                    let arr = self.arrays.get(arr_id);
                    for item in arr {
                        search_queue.push_back(*item);
                    }
                    completed_arrays.insert(*arr_id);
                }
                Value::Object(obj_id) => {
                    let obj = self.objects.get(obj_id);
                    for value in obj.values() {
                        search_queue.push_back(*value);
                    }
                    completed_objects.insert(*obj_id);
                }
                _ => (),
            }

            completed.insert(value);
        }

        self.values
            .filter_keys(completed.into_iter().collect::<Vec<_>>().as_slice());
        self.arrays
            .filter_keys(completed_arrays.into_iter().collect::<Vec<_>>().as_slice());
        self.objects
            .filter_keys(completed_objects.into_iter().collect::<Vec<_>>().as_slice());
    }

    /// Get the number of [Value]'s in the stack
    pub fn stack_size(&self) -> usize {
        self.stack.value_len()
    }

    /// Get the number of values in the value heap
    pub fn value_heap_size(&self) -> usize {
        self.values.len()
    }

    /// Get the number of arrays in the array heap
    pub fn array_heap_size(&self) -> usize {
        self.arrays.len()
    }

    /// Get the number of objects in the object heap
    pub fn object_heap_size(&self) -> usize {
        self.objects.len()
    }
}
