#![warn(clippy::pedantic)]

mod context;
mod error;
mod stack;
mod value;

pub use context::{Context, Returnable};
pub use error::Error;
pub use value::Value;
