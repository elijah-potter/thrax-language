use parser;

macro_rules! create_test {
    ($filename:ident) => {
       paste::paste! {
           #[test]
           fn [<parses_$filename>](){
                let source = include_str!(concat!("./tests_sources/", stringify!($filename), ".la"));

                parser::parse_string(&source).unwrap();
           }
       }
    };
}

create_test!(while_loop);
create_test!(fib);
create_test!(empty_fn);
create_test!(add_fns);
