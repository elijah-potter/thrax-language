use interpreter::{Context, BlockExit};

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

create_test!(while_loop, BlockExit::Completed);
create_test!(fib, BlockExit::Returned(Some(_)));
create_test!(empty_fn, BlockExit::Completed);
create_test!(add_fns, BlockExit::Completed);
create_test!(cyclic_arrays, BlockExit::Completed);
create_test!(index_object, BlockExit::Completed);
create_test!(timing, BlockExit::Completed);
create_test!(break_continue, BlockExit::Returned(Some(_)));
