use crate::{Context, Error, ShallowValue, Value};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_function("println".to_string(), |args| {
        for arg in args {
            print!("{}\t", arg);
        }
        println!();
        Ok(Value::Null)
    });
    context.add_native_function("push".to_string(), |args| {
        if args.len() < 2 {
            return Err(Error::IncorrectArgumentCount(2, args.len()));
        }

        let mut args_iter = args.iter();

        let first = args_iter.next().unwrap();
        let Value::Array(arr) = first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
            };
        let mut owned_arr = arr.borrow_mut();

        for arg in args_iter {
            owned_arr.push(arg.clone())
        }

        Ok(Value::Null)
    });
}
