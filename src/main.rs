use std::{
    fs::File,
    io::{stdin, stdout, BufRead, Read, Write},
    path::PathBuf,
};

use clap::{ArgAction::SetTrue, Parser};

mod ast;
mod environment;
mod error;
mod tokens;

use crate::{environment::Environment, error::Error, tokens::TokenCtx};

#[derive(Parser)]
#[command(about = "lox - from crafting interpreters")]
struct CliOptions {
    #[arg()]
    input_file: Option<PathBuf>,
    #[arg(short, long, default_value_t = false, action = SetTrue)]
    debug: bool,
}

fn main() {
    let args = CliOptions::parse();

    if let Some(file) = args.input_file {
        run_file(&file, args.debug);
    } else {
        run_interpreter(args.debug);
    }
}

fn print_errs(src: &str, err: std::slice::IterMut<Error>, contextualise: bool, interpreted: bool) {
    for e in err {
        if contextualise {
            e.contextualize(src);
        }
        e.report(interpreted);
    }

    if !interpreted {
        std::process::exit(65);
    }
}

fn run(src: &str, env: &mut Environment, interpreted: bool, debug: bool) {
    let tokens = match TokenCtx::from_str(src) {
        Ok(tokens) => tokens.into_boxed_slice(),
        Err(mut err) => {
            print_errs(src, err.iter_mut(), false, interpreted);
            return;
        }
    };

    if debug {
        println!("{:?}", tokens);
    }

    let stmts = match ast::scanner::Scanner::new(tokens).parse() {
        Ok(stmts) => stmts,
        Err(mut err) => {
            print_errs(src, err.iter_mut(), true, interpreted);
            return;
        }
    };

    if debug {
        println!("{:?}", stmts);
    }

    for stmt in stmts {
        if let Err(err) = stmt.parse(env, interpreted) {
            print_errs(src, vec![err].iter_mut(), true, interpreted);
        }
    }
}

fn run_file(path: &PathBuf, debug: bool) {
    let mut src = String::new();
    let mut environment = Environment::new(None);
    File::open(path)
        .expect("Unable to open file")
        .read_to_string(&mut src)
        .expect("Unable to read file");
    src.push('\0');

    run(&src, &mut environment, false, debug);
}

fn run_interpreter(debug: bool) {
    let mut reader = stdin().lock();
    let mut line = String::new();
    let mut environment = Environment::new(None);

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

        run(&line, &mut environment, true, debug);

        line.clear();
    }
}
