#![doc = include_str!("../README.md")]

mod context;
mod error;
mod stack;
mod stdlib;
mod value;

pub use context::{BlockExit, Context};
pub use error::Error;
pub use value::{GcValue, NativeFn, ShallowValue, Value};
