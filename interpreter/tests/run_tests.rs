use parser;
use interpreter::{Context, Returnable};


macro_rules! create_test {
    ($filename:ident, $e:pat) => {
       paste::paste! {
           #[test]
           fn [<parses_$filename>](){
                let source = include_str!(concat!("./tests_sources/", stringify!($filename), ".la"));

                let ast = parser::parse_string(&source).unwrap();
                let mut context = Context::new();

                assert!(matches!(context.eval_program(&ast), Ok($e)));
           }
       }
    };
}

create_test!(while_loop, Returnable::Completed);
create_test!(fib, Returnable::Returned(Some(interpreter::Value::Number(233.0))));
create_test!(empty_fn, Returnable::Completed);
create_test!(add_fns, Returnable::Completed);
