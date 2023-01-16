use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Context, Error, ShallowValue, Value};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_function("timestamp".to_string(), |context, args| {
        let time_in_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Ok(Value::Number(time_in_ms as f64).into_gc())
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
            arr.push_back(arg.clone())
        }

        Ok((Value::Null).into_gc())
    });

    context.add_native_function("pop".to_string(), |_context, args| {
        if args.is_empty() {
            return Err(Error::IncorrectArgumentCount(1, args.len()));
        }

        let mut args_iter = args.iter();

        let mut first = (*args_iter.next().unwrap()).borrow_mut();

        let Value::Array(arr) = &mut *first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
        };

        Ok(arr.pop_back().unwrap_or_else(|| Value::Null.into_gc()))
    });

    context.add_native_function("unshift".to_string(), |_context, args| {
        if args.len() < 2 {
            return Err(Error::IncorrectArgumentCount(2, args.len()));
        }

        let mut args_iter = args.iter();

        let mut first = (*args_iter.next().unwrap()).borrow_mut();

        let Value::Array(arr) = &mut *first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
        };

        for arg in args_iter {
            arr.push_front(arg.clone())
        }

        Ok((Value::Null).into_gc())
    });

    context.add_native_function("shift".to_string(), |_context, args| {
        if args.is_empty() {
            return Err(Error::IncorrectArgumentCount(1, args.len()));
        }

        let mut args_iter = args.iter();

        let mut first = (*args_iter.next().unwrap()).borrow_mut();

        let Value::Array(arr) = &mut *first else{
                return Err(Error::TypeError(ShallowValue::Array, first.as_shallow()));
        };

        Ok(arr.pop_front().unwrap_or_else(|| Value::Null.into_gc()))
    });

    context.add_native_function("len".to_string(), |_context, args| {
        if args.is_empty() {
            return Err(Error::IncorrectArgumentCount(1, args.len()));
        }

        let mut args_iter = args.iter();

        let mut first = (*args_iter.next().unwrap()).borrow_mut();

        let len = match &mut *first {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => return Err(Error::CannotIndexType(first.as_shallow())),
        };

        Ok(Value::Number(len as f64).into_gc())
    });
}
