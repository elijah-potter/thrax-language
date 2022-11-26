use interpreter::{Context, Returnable};
use parser;

macro_rules! create_test {
    ($filename:ident, $e:pat) => {
       paste::paste! {
           #[test]
           fn [<runs_$filename>](){
                let source = include_str!(concat!("./tests_sources/", stringify!($filename), ".la"));

                let ast = parser::parse_string(&source).unwrap();
                let mut context = Context::new();
                context.add_stdlib();

                assert!(matches!(context.eval_program(&ast), Ok($e)));
           }
       }
    };
}

create_test!(while_loop, Returnable::Completed);
create_test!(fib, Returnable::Returned(Some(_)));
create_test!(empty_fn, Returnable::Completed);
create_test!(add_fns, Returnable::Completed);
