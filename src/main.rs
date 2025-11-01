use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write, stdin, stdout},
    path::PathBuf,
};

use clap::Parser;

mod ast;
mod error;
mod tokens;

use crate::tokens::Token;

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
    let tokens = Token::from_str(src);
    match tokens {
        Ok(tokens) => println!("{:?}", tokens),
        Err(errors) => {
            errors.iter().for_each(|e| e.report(interpreted));
        }
    }
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
    let mut reader = BufReader::new(stdin().lock());
    let mut line = String::new();

    loop {
        print!(">> ");
        stdout().flush().expect("Error flushing output!");
        reader.read_line(&mut line).expect("Error reading input!");

        if line.is_empty() {
            break;
        }

        line.push('\0');

        run(&line, true);

        line.clear();
    }
}
