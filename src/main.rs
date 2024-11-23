use std::io;

use app_context::Context;
use evaluator::evaluate;
use tokenizer::tokenize;
use parser::{shunting_yard, validate};

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;
pub mod app_context;

fn main() {
    loop {
        let context = Context::new();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let mut tokens = match tokenize(input.trim()) {
            Ok(val) => val,
            Err(e) => {
                println!("Tokenizer error: {}", e);
                continue;
            }
        };

        if !validate(&tokens) {
            println!("Parser error");
            continue;
        }

        tokens = shunting_yard(tokens);
        match evaluate(tokens, &context) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Evaluation error: {e}"),
        }
    }
}