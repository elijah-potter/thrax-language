use crate::{Context, Error, ShallowValue, Value};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_function("println".to_string(), |context, args| {
        for arg in args {
            println!("{}", value_to_string(context, arg)?);
        }
        println!();
        Ok(Value::Null)
    });
    context.add_native_function("push".to_string(), |context, args| {
        if args.len() < 2 {
            return Err(Error::IncorrectArgumentCount(2, args.len()));
        }

        let mut args_iter = args.iter();

        let first = args_iter.next().unwrap();
        let Value::Array(arr_id) = first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
            };

        let owned_arr = context.get_array_mut(arr_id)?;

        for arg in args_iter {
            owned_arr.push(arg.clone())
        }

        Ok(Value::Null)
    });
}
/// Convert a [Value] to a human readable [String]
pub fn value_to_string(context: &Context, value: &Value) -> Result<String, Error> {
    // This is not elegant

    match value {
        Value::Number(n) => Ok(format!("{}", n)),
        Value::String(s) => Ok(format!("{}", s)),
        Value::Bool(b) => Ok(format!("{}", b)),
        Value::Array(arr_id) => {
            let mut s = String::new();
            s.push('[');

            let arr = context.get_array(arr_id)?;

            if arr.len() > 1 {
                for item in arr.iter().take(arr.len() - 1) {
                    s.push_str(value_to_string(context, item)?.as_str());
                    s.push_str(", ");
                }
            }

            if let Some(item) = arr.last() {
                s.push_str(value_to_string(context, item)?.as_str());
            }

            s.push(']');

            Ok(s)
        }
        Value::Fn(_) => Ok("Function".to_string()),
        Value::Null => Ok("Null".to_string()),
    }
}
