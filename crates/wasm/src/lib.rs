mod println;

use std::rc::Rc;

use interpreter::{Callable, GcCell, NativeFn, Value};
use js_sys::Function;
use println::PrintLn;
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsValue};

#[wasm_bindgen]
pub struct Context {
    inner: interpreter::Context,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new(log_fn: Function) -> Context {
        let mut inner = interpreter::Context::new();
        inner.add_stdlib();

        inner.add_callable("println", Rc::new(GcCell::new(PrintLn::new(log_fn))));

        Self { inner }
    }

    pub fn easy_eval(&mut self, program: &str) -> String {
        let program = match parser::parse_string(program) {
            Ok(program) => program,
            Err(err) => throw_str(&err.to_string()),
        };

        match self.inner.eval_program(&program) {
            Err(err) => throw_str(&err.to_string()),
            Ok(returned) => format!(
                "{}",
                returned
                    .returned()
                    .flatten()
                    .unwrap_or(Value::Null.into_gc())
            ),
        }
    }
}
