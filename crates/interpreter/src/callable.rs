use ast::Stmt;
use gc::{Finalize, Trace};

use crate::{BlockExit, Context, Error, GcValue, Value};

pub trait Callable: Trace + Finalize {
    fn call(&self, context: &mut Context, args: &[GcValue]) -> Result<GcValue, Error>;
}

#[derive(Debug, Clone, Trace, Finalize)]
pub struct InterpretedFn {
    stack_height: usize,
    prop_idents: Vec<String>,
    #[unsafe_ignore_trace]
    body: Vec<Stmt>,
}

impl InterpretedFn {
    pub fn new(stack_height: usize, prop_idents: Vec<String>, body: Vec<Stmt>) -> Self {
        Self {
            stack_height,
            prop_idents,
            body,
        }
    }
}

impl Callable for InterpretedFn {
    fn call(&self, context: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
        if args.len() != self.prop_idents.len() {
            return Err(Error::IncorrectArgumentCount(
                self.prop_idents.len(),
                args.len(),
            ));
        }

        let popped = context.stack.pop_until_index(self.stack_height);
        context.stack.open_frame();

        for (ident, value) in self.prop_idents.iter().zip(args.iter()) {
            context.stack.push_value(ident.clone(), value.clone());
        }

        let res = context.eval_program(&self.body)?;

        context.stack.pop_frame();
        context.stack.push_popped_stack(popped);

        if let BlockExit::Returned(r) = res {
            return Ok(r.unwrap_or_else(|| (Value::Null).into_gc()));
        }

        Ok((Value::Null).into_gc())
    }
}

#[derive(Trace, Finalize)]
pub struct NativeFn(
    #[unsafe_ignore_trace] pub fn(context: &mut Context, args: &[GcValue]) -> Result<GcValue, Error>,
);

impl Callable for NativeFn {
    fn call(&self, context: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
        self.0(context, args)
    }
}

// pub type NativeFn = fn(&mut Context, &[GcValue]) -> Result<GcValue, Error>;

// #[derive(Trace, Finalize)]
// pub enum Fn {
//     Native(#[unsafe_ignore_trace] NativeFn),
//     /// This is only expressly different from `ast::FnDecl` in that it does not include an ident.
//     /// TODO: Store stack frame alongside function
//     Interpreted {
//         #[unsafe_ignore_trace]
//         prop_idents: Vec<String>,
//         #[unsafe_ignore_trace]
//         body: Vec<Stmt>,
//     },
// }
