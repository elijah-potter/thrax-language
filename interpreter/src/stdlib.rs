use crate::{Context, Error, ShallowValue, Value};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_function("println".to_string(), |context, args| {
        for arg in args {
            print!("{}", value_to_string(context, &arg.borrow()));
        }
        println!();
        Ok((Value::Null).into_gc())
    });
    context.add_native_function("push".to_string(), |_context, args| {
        if args.len() < 2 {
            return Err(Error::IncorrectArgumentCount(2, args.len()));
        }

        let mut args_iter = args.iter();

        let mut first = (*args_iter.next().unwrap()).borrow_mut();

        let Value::Array(arr) = &mut *first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
            };

        for arg in args_iter {
            arr.push(arg.clone())
        }

        Ok((Value::Null).into_gc())
    });
}
/// Convert a [Value] to a human readable [String]
pub fn value_to_string(context: &Context, value: &Value) -> String {
    // This is not elegant

    match value {
        Value::Number(n) => format!("{}", n),
        Value::String(s) => s.to_string(),
        Value::Bool(b) => format!("{}", b),
        Value::Array(arr) => {
            let mut s = String::new();
            s.push('[');

            if arr.len() > 1 {
                for item in arr.iter().take(arr.len() - 1) {
                    s.push_str(value_to_string(context, &item.borrow()).as_str());
                    s.push_str(", ");
                }
            }

            if let Some(item) = arr.last() {
                s.push_str(value_to_string(context, &item.borrow()).as_str());
            }

            s.push(']');

            s
        }
        Value::Object(obj) => {
            let mut s = String::new();

            s.push('{');

            for (key, value) in obj.iter() {
                s.push_str(key);

                s.push_str(": ");

                s.push_str(value_to_string(context, &value.borrow()).as_str());

                s.push_str(", ");
            }

            s.push('}');

            s
        }
        Value::Fn(_) => "Function".to_string(),
        Value::Null => "Null".to_string(),
    }
}
