# Interpreter

## Quick Start

The core of this crate is the `Context` struct.

The simplest usage, assuming you have a parsed program is as follows:

```rust
let source = "1 + 2;";

// See the parser crate for more information on this.
let program = parser::parse_string(source).unwrap();

let mut context = interpreter::Context::new();

// Many of the core functions of the language require this (like pushing and popping from arrays),
// but if you want to build a DSL, you don't have to do this.
context.add_stdlib();

context.eval_program(&program).unwrap();
```

## Adding Native Functions

You can add Rust functions to the interpreter [`Context`] with [`Context::add_native_function`].

```rust
use interpreter::{Error, Context, GcValue, NativeFn};

let mut context = Context::new();

let add_fn = |context: &mut Context, arguments: &[GcValue]| {
  // We are only looking at the first two arguments, but there can be an unlimited number.
  if arguments.len() < 2{
    return Err(Error::IncorrectArgumentCount(2, arguments.len()));
  }

  let a = arguments[0].borrow();
  let b = arguments[1].borrow();

  Ok(a.add(&b)?.into_gc())
};

context.add_native_fn("add".to_string(), NativeFn(add_fn));
```

## Examples

The best example of using this crate is the CLI, which can be found in the `crates` directory of the main repo.

Specifically, for an example of adding native functions, see the `add_io` function inside the CLI `main.rs`.
