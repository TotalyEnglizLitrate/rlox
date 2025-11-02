use std::{
    fs::File,
    io::{BufRead, Read, Write, stdin, stdout},
    path::PathBuf,
};

use clap::Parser;

mod ast;
mod error;
mod tokens;

use crate::tokens::{Token, TokenCtx};

#[derive(Parser)]
#[command(about = "lox - from crafting interpreters")]
struct CliOptions {
    #[arg()]
    input_file: Option<PathBuf>,
}

fn main() {
    let args = CliOptions::parse();

    if let Some(file) = args.input_file {
        run_file(&file);
    } else {
        run_interpreter();
    }
}

fn run(src: &str, interpreted: bool) {
    let tokens = match TokenCtx::from_str(src) {
        Ok(tokens) => tokens.into_boxed_slice(),
        Err(errors) => {
            errors.iter().for_each(|e| e.report(interpreted));
            if interpreted {
                return;
            } else {
                std::process::exit(65);
            }
        }
    };

    println!("{:?}", tokens);

    let ast = match ast::scanner::Scanner::new(tokens).parse_expression() {
        Ok(Some(ast)) => ast,
        Ok(None) => return,
        Err(mut errors) => {
            errors.iter_mut().for_each(|e| {
                e.contextualize(src);
                e.report(interpreted);
            });
            if interpreted {
                return;
            } else {
                std::process::exit(65);
            }
        }
    };

    println!("{:?}", ast);
}

fn run_file(path: &PathBuf) {
    let mut src = String::new();
    File::open(path)
        .expect("Unable to open file")
        .read_to_string(&mut src)
        .expect("Unable to read file");
    src.push('\0');

    run(&src, false);
}

fn run_interpreter() {
    let mut reader = stdin().lock();
    let mut line = String::new();

    loop {
        print!(">> ");
        stdout().flush().expect("Error flushing output!");

        match reader.read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }

            Ok(_) => {
                line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }
                line.push('\0');
            }

            Err(_) => {
                eprintln!("Error reading input!");
                break;
            }
        }

        run(&line, true);

        line.clear();
    }
}
