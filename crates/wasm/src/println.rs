use gc::{Finalize, Trace};
use interpreter::{Callable, Value};
use js_sys::Function;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Trace, Finalize)]
pub struct PrintLn {
    #[unsafe_ignore_trace]
    inner_log: Function,
}

impl PrintLn {
    pub fn new(inner_log: Function) -> Self {
        Self { inner_log }
    }
}

impl Callable for PrintLn {
    fn call(
        &self,
        _context: &mut interpreter::Context,
        args: &[interpreter::GcValue],
    ) -> Result<interpreter::GcValue, interpreter::Error> {
        let mut line = String::new();

        for arg in args {
            line.push_str(&format!("{}", arg));
        }

        let _ = self
            .inner_log
            .call1(&JsValue::NULL, &JsValue::from_str(&line));

        Ok((Value::Null).into_gc())
    }
}
