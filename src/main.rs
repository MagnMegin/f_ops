use std::io::{self, Write};

use app_context::Context;
use evaluator::{evaluate, EvalOutput};
use tokenizer::tokenize;
use parser::{shunting_yard, validate};

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;
pub mod app_context;

fn main() {
    let mut context = Context::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout.flush().unwrap();
        
        stdin.read_line(&mut input).unwrap();
        
        let mut tokens = match tokenize(input.trim()) {
            Ok(val) => val,
            Err(e) => {
                println!("{e}\n");
                continue;
            }
        };

        if let Err(e) = validate(&tokens) {
            println!("{e}\n");
            continue;
        }

        tokens = shunting_yard(tokens);
        match evaluate(tokens, &mut context) {
            Ok(result) => match result {
                EvalOutput::Assignment(var, val) => println!("Assigned {val} to {var}\n"),
                EvalOutput::Value(value) => println!("{value}\n"),
            },
            Err(e) => println!("{e}\n"),
        }
    }
}