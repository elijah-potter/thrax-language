#![warn(clippy::pedantic)]

use std::fs::read;
use std::path::PathBuf;

use ast::Program;
use clap::{Parser, Subcommand};
use interpreter::{Context, Returnable, ShallowValue, Value};
use parser::{lex_string, parse_tokens};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Run {
        filename: PathBuf,
        #[arg(long, short, default_value_t = false)]
        no_gc: bool,
    },
    Ast {
        filename: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Action::Run { filename, no_gc } => {
            let ast = load_ast(&filename).expect("Could not load AST");

            let mut context = Context::new(!no_gc);
            context.add_stdlib();

            match context.eval_program(&ast) {
                Err(err) => println!("{:#?}", err),
                Ok(Returnable::Returned(Some(v))) => {
                    println!("{}", v.as_shallow())
                }
                _ => (),
            }

            eprintln!("FINAL STACK SIZE: {}", context.stack_size());
            eprintln!("FINAL HEAP SIZE: {}", context.array_heap_size());
        }
        Action::Ast { filename } => {
            let ast = load_ast(&filename).expect("Could not load AST");
            eprintln!("{:#?}", ast);
        }
    }
}

fn load_ast(filename: &PathBuf) -> Option<Program> {
    let file = read(filename).expect("Could not read file.");
    let source = String::from_utf8(file).expect("Could not parse UTF-8 source.");

    let tokens = match lex_string(&source) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{:#?}", err);
            return None;
        }
    };

    match parse_tokens(&tokens) {
        Ok(ast) => return Some(ast),
        Err(err) => {
            eprintln!("{:#?}", err);
            return None;
        }
    };
}
