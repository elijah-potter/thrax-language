# Parser

## Quick Start

This crate is fairly straightforward to use.

To quickly parse a string into an executable (via the interpreter) AST, use [`parse_string`]:

```rust
let source = r#"
  fn greet(name){
    return "Hello " + name;
  }

  return greet("world!");
"#;

let program = parser::parse_string(source).unwrap();

// ... run it or something
# let mut context = interpreter::Context::new();
# context.eval_program(&program).unwrap();
```

While the above is what you likely want to do in most situations,
you can also use the individual [`lex_string`] and [`parse_tokens`] functions.
