#![doc = include_str!("../README.md")]

mod callable;
mod context;
mod error;
mod stack;
mod stdlib;
mod value;

pub use callable::{Callable, InterpretedFn, NativeFn};
pub use context::{BlockExit, Context};
pub use error::Error;
pub use value::{GcValue, ShallowValue, Value};
