use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

use ast::{BinaryOpKind, FnCall, Stmt};
use gc::{unsafe_empty_trace, Finalize, Gc, GcCell, GcCellRef, GcCellRefMut, Trace};

use crate::error::Error;
use crate::{Callable, Context, NativeFn};

#[derive(Clone, Trace, Finalize)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Array(VecDeque<GcValue>),
    Object(HashMap<String, GcValue>),
    Callable(Rc<GcCell<dyn Callable>>),
    Null,
}

impl Value {
    pub fn as_shallow(&self) -> ShallowValue {
        match self {
            Value::Number(_) => ShallowValue::Number,
            Value::String(_) => ShallowValue::String,
            Value::Bool(_) => ShallowValue::Bool,
            Value::Array(_) => ShallowValue::Array,
            Value::Object(_) => ShallowValue::Object,
            Value::Callable(_) => ShallowValue::Callable,
            Value::Null => ShallowValue::Null,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShallowValue {
    Number,
    String,
    Bool,
    Array,
    Object,
    Callable,
    Null,
}

impl Display for ShallowValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            ShallowValue::Number => "Number",
            ShallowValue::String => "String",
            ShallowValue::Bool => "Bool",
            ShallowValue::Array => "Array",
            ShallowValue::Object => "Object",
            ShallowValue::Callable => "Callable",
            ShallowValue::Null => "Null",
        };

        write!(f, "{m}")
    }
}

#[derive(Clone, Trace, Finalize)]
pub struct GcValue {
    inner: Gc<GcCell<Value>>,
}

impl GcValue {
    pub fn new(value: Value) -> Self {
        Self {
            inner: Gc::new(GcCell::new(value)),
        }
    }

    pub fn borrow(&self) -> GcCellRef<'_, Value> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> GcCellRefMut<'_, Value> {
        self.inner.borrow_mut()
    }

    /// For when you want to pass a value either by referance or by value depending on it's type.
    pub fn shallow_copy(&self) -> Self {
        match &*self.borrow() {
            Value::Number(n) => Value::Number(*n).into_gc(),
            Value::String(s) => Value::String(s.clone()).into_gc(),
            Value::Bool(b) => Value::Bool(*b).into_gc(),
            Value::Null => Value::Null.into_gc(),
            _ => self.clone(),
        }
    }
}

impl Display for GcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.borrow().deref() {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Array(arr) => {
                let mut s = String::new();
                s.push('[');

                if arr.len() > 1 {
                    for item in arr.iter().take(arr.len() - 1) {
                        s.push_str(format!("{item}, ").as_str())
                    }
                }

                if let Some(item) = arr.iter().last() {
                    s.push_str(format!("{item}").as_str())
                }

                s.push(']');

                write!(f, "{s}")
            }
            Value::Object(obj) => {
                let mut s = String::new();

                s.push('{');

                for (key, value) in obj.iter() {
                    s.push_str(key);

                    s.push_str(": ");

                    s.push_str(format!("{value}").as_str());

                    s.push_str(", ");
                }

                s.push('}');

                write!(f, "{s}")
            }
            Value::Callable(_) => write!(f, "Function"),
            Value::Null => write!(f, "Null"),
        }
    }
}

impl std::fmt::Debug for GcValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Value {
    pub fn into_gc(self) -> GcValue {
        GcValue::new(self)
    }
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
        Number => |a, b| Value::Number(a / b)
    );

    impl_op!(GreaterThan,
        Number => |a, b| Value::Bool(a > b)
    );

    impl_op!(Pow,
        Number => |a: &f64, b: &f64| Value::Number(a.powf(*b))
    );

    impl_op!(LessThan,
        Number => |a, b| Value::Bool(a < b)
    );

    impl_op!(Equals,
        Number => |a, b| Value::Bool(a == b),
        Bool => |a, b| Value::Bool(a == b),
        String => |a:&str, b:&str| Value::Bool(a.eq(b))
    );

    pub fn run_binary_op(&self, other: &Self, op: BinaryOpKind) -> Result<Self, Error> {
        match op {
            ast::BinaryOpKind::Add => self.add(other),
            ast::BinaryOpKind::Subtract => self.subtract(other),
            ast::BinaryOpKind::Multiply => self.multiply(other),
            ast::BinaryOpKind::Divide => self.divide(other),
            ast::BinaryOpKind::GreaterThan => self.greater_than(other),
            ast::BinaryOpKind::LessThan => self.less_than(other),
            ast::BinaryOpKind::Pow => self.pow(other),
            ast::BinaryOpKind::Equals => self.equals(other),
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<Rc<GcCell<dyn Callable>>> for Value {
    fn from(value: Rc<GcCell<dyn Callable>>) -> Self {
        Value::Callable(value)
    }
}
