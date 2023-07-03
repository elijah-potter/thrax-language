use std::fs::read;
use std::io::stderr;
use std::path::PathBuf;
use std::rc::Rc;

use ast::Program;
use clap::{Parser, Subcommand};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use interpreter::{BlockExit, Context, NativeFn, Value};
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
            let Some(ast) = load_ast(&filename) else{
                print_err("Could not load AST");
                return;
            };

            let mut context = Context::new();
            context.add_stdlib();
            add_io(&mut context);

            match context.eval_program(&ast) {
                Err(err) => println!("{:#?}", err),
                Ok(BlockExit::Returned(Some(_v))) => {
                    println!("{}", _v);
                }
                _ => (),
            }
        }
        Action::Ast { filename } => {
            let Some(ast) = load_ast(&filename) else{
                print_err("Could not load AST");
                return;
            };
            eprintln!("{:#?}", ast);
        }
    }
}

fn load_ast(filename: &PathBuf) -> Option<Program> {
    let Ok(file) = read(filename) else{
        print_err("Could not load file from disk.");
        return None;
    };

    let Ok(source) = String::from_utf8(file)else{
        print_err("Could not parse UTF-8 source.");
        return None;
    };

    process_ast(&source)
}

fn process_ast(source: &str) -> Option<Program> {
    let tokens = match lex_string(source) {
        Ok(tokens) => tokens,
        Err(err) => {
            let (line, col) = line_col_from_index(err.index, source).unwrap();

            println!("{:#?}", err);
            println!("At line {}, row {}", line, col);
            return None;
        }
    };

    match parse_tokens(&tokens) {
        Ok(ast) => Some(ast),
        Err(err) => {
            let token = &tokens[err.index];

            let (start_line, start_col) = line_col_from_index(token.span.start, source).unwrap();
            let (end_line, end_col) = line_col_from_index(token.span.end, source).unwrap();

            print_line_col(start_line, end_line, start_col, end_col, source);

            if err.is_recoverable {
                print!("Recoverable Error: ");
            } else {
                print!("Unrecoverable Error: ")
            }

            println!("{}", err.kind);
            println!("At line {}, row {}", start_line, start_col);
            None
        }
    }
}

/// Computes the line:column an index is at in source code.
/// Returns `None` if the index is outside the source.
fn line_col_from_index(index: usize, source: &str) -> Option<(usize, usize)> {
    let mut traversed = 0;

    for (nth_line, source_line) in source.lines().enumerate() {
        let line_len = source_line.chars().count();

        if (traversed..traversed + line_len).contains(&index) {
            return Some((nth_line + 1, index - traversed + 1));
        }

        traversed += line_len + 1;
    }

    None
}

fn print_line_col(
    start_line: usize,
    end_line: usize,
    start_col: usize,
    end_col: usize,
    source: &str,
) {
    let end_line_nr_str = format!("{}", start_line);
    let padding = " ".repeat(end_line_nr_str.len());

    let mut stderr = stderr();

    for (index, line) in source.lines().enumerate() {
        if start_line > 1 && index == start_line - 2 {
            execute!(
                stderr,
                SetForegroundColor(Color::Blue),
                Print(format!("{} |\n", padding)),
                ResetColor
            )
            .unwrap();
        } else if index == end_line {
            execute!(
                stderr,
                SetForegroundColor(Color::Blue),
                Print(format!("{} | ", padding)),
                SetForegroundColor(Color::Red),
                Print(format!(
                    "{}{}\n",
                    " ".repeat(start_col - 1),
                    "^".repeat(end_col - start_col)
                )),
                ResetColor
            )
            .unwrap();
        } else if index >= start_line - 1 && index < end_line {
            let nr_string = format!("{}", index + 1);
            execute!(
                stderr,
                SetForegroundColor(Color::Blue),
                Print(format!(
                    "{}{} |",
                    " ".repeat(padding.len() - nr_string.len()),
                    nr_string
                )),
                SetForegroundColor(Color::White),
                Print(format!(" {}\n", line)),
                ResetColor
            )
            .unwrap();
        }
    }
}

fn print_err(err: &str) {
    let mut stderr = stderr();

    execute!(
        stderr,
        SetForegroundColor(Color::Red),
        Print(err),
        Print("\n"),
        ResetColor
    )
    .unwrap();
}

fn add_io(context: &mut Context) {
    context.add_native_fn(
        "println".to_string(),
        NativeFn(|_context, args| {
            for arg in args {
                print!("{}", arg);
            }
            println!();
            Ok((Value::Null).into_gc())
        }),
    );

    context.add_native_fn(
        "print".to_string(),
        NativeFn(|_context, args| {
            for arg in args {
                print!("{}", arg);
            }
            Ok(Value::Null.into_gc())
        }),
    );
}
