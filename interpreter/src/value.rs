use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

use ast::{BinaryOpKind, Stmt};

use crate::error::Error;
use crate::heap::HeapItem;
use crate::Context;

/// [Value] is a dynamically typed nullable value.
///
/// A value that _can_ be undefined is expressed as an `Option<Value>`
macro_rules! define_value_types {
    ($inner:ty) => {
        _
    };
    ($($kind:ident$(($contains:ty))?),*) => {
        #[derive(Clone)]
        pub enum Value {
            $(
                $kind $(($contains))?,
            )*
        }

        impl Value {
            #[must_use] pub fn as_shallow(&self) -> ShallowValue{
                match self{
                    $(
                        Self::$kind $( (define_value_types!($contains)) )? => ShallowValue::$kind,
                    )*
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ShallowValue{
            $(
                $kind,
            )*
        }

        impl Display for ShallowValue{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self{
                    $(
                        Self::$kind => write!(f, stringify!($kind)),
                    )*
                }
            }
        }
    };
}

define_value_types! {
    Number(f64),
    String(String),
    Bool(bool),
    Array(HeapItem<Vec<Value>>),
    Object(HeapItem<HashMap<String, Value>>),
    Fn(Rc<Fn>),
    Null
}

#[derive(Clone)]
pub enum Fn {
    Native(fn(&mut Context, &[Value]) -> Result<Value, Error>),
    /// This is only expressly different from `ast::FnDecl` in that it does not include an ident.
    /// TODO: Store stack frame alongside function
    Interpreted {
        prop_idents: Vec<String>,
        body: Vec<Stmt>,
    },
}

macro_rules! impl_op {
    ($op_kind:ident, $($variant:ident => $op:expr),*) => {
        paste::paste!{
            pub fn [<$op_kind:snake>](&self, other: &Self) -> Result<Self, Error>{
                let res = match self{
                    $(
                        Value::$variant(a) => {
                            let b = match other{
                                Value::$variant(b) => b,
                                _ => return Err(Error::TypeError(ShallowValue::$variant, other.as_shallow()))
                            };

                            let v = $op;

                            v(a, b)
                        },
                    )*
                    _ => return Err(Error::InvalidBinaryOpArgs(self.as_shallow(), other.as_shallow(), BinaryOpKind::$op_kind))
                };

                Ok(res)
            }
        }
    };
}

impl Value {
    impl_op!(Add,
        Number => |a, b| Value::Number(a + b),
        String => |a: &str, b: &str| {
            let mut s = String::with_capacity(a.len() + b.len());
            s.push_str(a);
            s.push_str(b);
            Value::String(s)
        }
    );

    impl_op!(Subtract,
        Number => |a, b| Value::Number(a - b)
    );

    impl_op!(Multiply,
        Number => |a, b| Value::Number(a * b)
    );

    impl_op!(Divide,
        Number => |a, b| Value::Number(a * b)
    );

    impl_op!(GreaterThan,
        Number => |a, b| Value::Bool(a > b)
    );

    impl_op!(LessThan,
        Number => |a, b| Value::Bool(a < b)
    );

    impl_op!(Equals,
        Number => |a, b| Value::Bool(a == b),
        Bool => |a, b| Value::Bool(a == b),
        String => |a:&str, b:&str| Value::Bool(a.eq(b))
    );
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
