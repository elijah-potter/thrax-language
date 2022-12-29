use crate::{Context, Error, ShallowValue, Value};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_function("println".to_string(), |context, args| {
        for arg in args {
            let arg = context.values.get(*arg);
            print!("{}", value_to_string(context, arg));
        }
        println!();
        Ok(context.values.push(Value::Null))
    });
    context.add_native_function("push".to_string(), |context, args| {
        if args.len() < 2 {
            return Err(Error::IncorrectArgumentCount(2, args.len()));
        }

        let mut args_iter = args.iter();

        let first = context.values.get(*args_iter.next().unwrap());

        let Value::Array(arr_id) = first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
            };

        let owned_arr = context.arrays.get_mut(*arr_id);

        for arg in args_iter {
            owned_arr.push(arg.clone())
        }

        Ok(context.values.push(Value::Null))
    });
}
/// Convert a [Value] to a human readable [String]
pub fn value_to_string(context: &Context, value: &Value) -> String {
    // This is not elegant

    match value {
        Value::Number(n) => format!("{}", n),
        Value::String(s) => s.to_string(),
        Value::Bool(b) => format!("{}", b),
        Value::Array(arr_id) => {
            let mut s = String::new();
            s.push('[');

            let arr = context.arrays.get(*arr_id);

            if arr.len() > 1 {
                for item in arr.iter().take(arr.len() - 1) {
                    let inner_item = context.values.get(*item);
                    s.push_str(value_to_string(context, inner_item).as_str());
                    s.push_str(", ");
                }
            }

            if let Some(item) = arr.last() {
                let inner_item = context.values.get(*item);
                s.push_str(value_to_string(context, inner_item).as_str());
            }

            s.push(']');

            s
        }
        Value::Object(obj_id) => {
            let mut s = String::new();

            s.push('{');

            let obj = context.objects.get(*obj_id);

            for (key, value) in obj.iter() {
                s.push_str(key);

                s.push_str(": ");

                let inner_value = context.values.get(*value);

                s.push_str(value_to_string(context, inner_value).as_str());

                s.push_str(", ");
            }

            s.push('}');

            s
        }
        Value::Fn(_) => "Function".to_string(),
        Value::Null => "Null".to_string(),
    }
}
