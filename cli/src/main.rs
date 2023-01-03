use std::fs::read;
use std::path::PathBuf;

use ast::Program;
use clap::{Parser, Subcommand};
use interpreter::{Context, Returnable};
use parser::{lex_string, parse_tokens};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Run { filename: PathBuf },
    Ast { filename: PathBuf },
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        Action::Run { filename } => {
            let ast = load_ast(&filename).expect("Could not load AST");

            let mut context = Context::new();
            context.add_stdlib();

            match context.eval_program(&ast) {
                Err(err) => println!("{:#?}", err),
                Ok(Returnable::Returned(Some(_v))) => {}
                _ => (),
            }
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
        Ok(ast) => Some(ast),
        Err(err) => {
            eprintln!("{:#?}", err);
            None
        }
    }
}
