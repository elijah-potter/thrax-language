use crate::{Context, Error, GcValue, NativeFn, ShallowValue, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn add_stdlib(context: &mut Context) {
    context.add_native_fn("timestamp".to_string(), NativeFn(timestamp));

    context.add_native_fn("push".to_string(), NativeFn(push));

    context.add_native_fn("pop".to_string(), NativeFn(pop));

    context.add_native_fn("unshift".to_string(), NativeFn(unshift));

    context.add_native_fn("shift".to_string(), NativeFn(shift));

    context.add_native_fn("len".to_string(), NativeFn(len));
}

fn timestamp(_ctx: &mut Context, _args: &[GcValue]) -> Result<GcValue, Error> {
    let time_in_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    Ok(Value::Number(time_in_ms as f64).into_gc())
}

fn push(_ctx: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
    if args.len() < 2 {
        return Err(Error::IncorrectArgumentCount(2, args.len()));
    }

    let mut args_iter = args.iter();

    let mut first = (*args_iter.next().unwrap()).borrow_mut();

    let Value::Array(arr) = &mut *first else {
        return Err(Error::TypeError(ShallowValue::Array, (*first).as_shallow()));
    };

    for arg in args_iter {
        arr.push_back(arg.clone())
    }

    Ok((Value::Null).into_gc())
}

fn pop(_ctx: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
    if args.is_empty() {
        return Err(Error::IncorrectArgumentCount(1, args.len()));
    }

    let mut args_iter = args.iter();

    let mut first = (*args_iter.next().unwrap()).borrow_mut();

    let Value::Array(arr) = &mut *first else {
        return Err(Error::TypeError(ShallowValue::Array, (*first).as_shallow()));
    };

    Ok(arr.pop_back().unwrap_or_else(|| Value::Null.into_gc()))
}
fn unshift(_ctx: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
    if args.len() < 2 {
        return Err(Error::IncorrectArgumentCount(2, args.len()));
    }

    let mut args_iter = args.iter();

    let mut first = (*args_iter.next().unwrap()).borrow_mut();

    let Value::Array(arr) = &mut *first else {
        return Err(Error::TypeError(ShallowValue::Array, (*first).as_shallow()));
    };

    for arg in args_iter {
        arr.push_front(arg.clone())
    }

    Ok((Value::Null).into_gc())
}
fn shift(_ctx: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
    if args.is_empty() {
        return Err(Error::IncorrectArgumentCount(1, args.len()));
    }

    let mut args_iter = args.iter();

    let mut first = (*args_iter.next().unwrap()).borrow_mut();

    let Value::Array(arr) = &mut *first else {
        return Err(Error::TypeError(ShallowValue::Array, (*first).as_shallow()));
    };

    Ok(arr.pop_front().unwrap_or_else(|| Value::Null.into_gc()))
}

fn len(_ctx: &mut Context, args: &[GcValue]) -> Result<GcValue, Error> {
    if args.is_empty() {
        return Err(Error::IncorrectArgumentCount(1, args.len()));
    }

    let mut args_iter = args.iter();

    let mut first = (*args_iter.next().unwrap()).borrow_mut();

    let len = match &mut *first {
        Value::String(s) => s.len(),
        Value::Array(a) => a.len(),
        Value::Object(o) => o.len(),
        _ => return Err(Error::CannotIndexType((*first).as_shallow())),
    };

    Ok(Value::Number(len as f64).into_gc())
}
