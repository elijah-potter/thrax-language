use interpreter::{NativeFn, Value};
use wasm_bindgen::{prelude::wasm_bindgen, throw_str};

#[wasm_bindgen]
pub struct Context {
    inner: interpreter::Context,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Context {
        let mut inner = interpreter::Context::new();
        inner.add_stdlib();

        inner.add_native_fn(
            "println".to_string(),
            NativeFn(|_context, args| {
                let mut line = String::new();

                for arg in args {
                    line.push_str(&format!("{}", arg));
                }

                log(&line);
                Ok((Value::Null).into_gc())
            }),
        );

        Self { inner }
    }

    pub fn easy_eval(&mut self, program: &str) {
        let program = match parser::parse_string(program) {
            Ok(program) => program,
            Err(err) => throw_str(&err.to_string()),
        };

        match self.inner.eval_program(&program) {
            Err(err) => throw_str(&err.to_string()),
            _ => (),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
