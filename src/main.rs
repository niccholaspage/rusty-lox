mod ast_printer;
mod expr;
mod literal;
mod parser;
mod scanner;
mod token;
mod token_type;
mod visitor;

use std::{
    cmp::Ordering,
    env, fs,
    io::{self, Write},
    process::exit, cell::RefCell,
};

use ast_printer::AstPrinter;
use parser::Parser;
use token::Token;
use token_type::TokenType;
use typed_arena::Arena;

use crate::scanner::Scanner;

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

    fn error_with_token(&mut self, token: &Token, message: &str) {
        if token.r#type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let context = RefCell::new(Context::new());

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rusty-lox [script]");
        }
        Ordering::Equal => {
            run_file(context, &args[1]);
        }
        Ordering::Less => {
            run_prompt(context);
        }
    }
}

fn run_file(context: RefCell<Context>, path: &str) {
    let content = fs::read(&path);

    match content {
        Ok(content) => {
            run(&context, content);

            if context.borrow().had_error {
                exit(65);
            }
        }
        Err(_) => {
            eprintln!("Error reading file!");

            exit(70);
        }
    };
}

fn run_prompt(context: RefCell<Context>) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        line.truncate(line.len() - 1);

        run(&context, line.into_bytes());

        context.borrow_mut().had_error = false;
    }
}

fn run(context: &RefCell<Context>, source: Vec<u8>) {
    let scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens(context);

    let arena = Arena::new();
    let mut parser = Parser::new(context, tokens);
    let expression = parser.parse(&arena);

    if context.borrow().had_error {
        return;
    }

    let expression = expression.unwrap();

    let mut printer = AstPrinter {};
    println!("{}", printer.print(expression));
}
