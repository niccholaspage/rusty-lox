mod literal;
mod scanner;
mod token;
mod token_type;

use std::{
    cmp::Ordering,
    env, fs,
    io::{self, Write},
    process::exit,
};

use scanner::Scanner;

pub struct Context {
    had_error: bool,
}

impl Context {
    fn new() -> Context {
        Context { had_error: false }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&mut self, line: usize, r#where: &str, message: &str) {
        eprintln!("[line {line}] Error{where}: {message}");
        self.had_error = true;
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut context = Context::new();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rusty-lox [script]");
        }
        Ordering::Equal => {
            run_file(&mut context, &args[1]);
        }
        Ordering::Less => {
            run_prompt(&mut context);
        }
    }
}

fn run_file(context: &mut Context, path: &str) {
    let content = fs::read(&path);

    match content {
        Ok(content) => {
            run(context, content);

            if context.had_error {
                exit(65);
            }
        }
        Err(_) => {
            eprintln!("Error reading file!");

            exit(70);
        }
    };
}

fn run_prompt(context: &mut Context) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        line.truncate(line.len() - 1);

        run(context, line.into_bytes());

        context.had_error = false;
    }
}

fn run(context: &mut Context, source: Vec<u8>) {
    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens(context);

    for token in tokens {
        println!("{}", token);
    }
}