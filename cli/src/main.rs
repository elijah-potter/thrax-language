#![warn(clippy::pedantic)]

use std::fs::read;
use std::path::PathBuf;

use clap::Parser;
use interpreter::{Context, Value};
use parser::{lex_string, parse_tokens};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: PathBuf,
    #[arg(short, long)]
    ast: bool,
    #[arg(short, long)]
    run: bool,
}

fn main() {
    let args = Args::parse();

    let file = read(args.filename).expect("Could not read file.");
    let source = String::from_utf8(file).expect("Could not parse UTF-8 source.");

    let tokens = match lex_string(&source) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{:#?}", err);
            return;
        }
    };

    let ast = match parse_tokens(&tokens) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{:#?}", err);
            return;
        }
    };

    if args.ast {
        eprintln!("{:#?}", ast);
    }

    if args.run {
        let mut context = Context::new();
        context.add_native_function("println".to_string(), |args| {
            for arg in args {
                print!("{}\t", arg);
            }
            println!();
            Ok(Value::Null)
        });

        if let Err(err) = context.eval_program(&ast) {
            println!("{:#?}", err);
        }
    }
}
