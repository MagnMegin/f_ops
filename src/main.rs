use std::io;

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
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let mut tokens = match tokenize(input.trim()) {
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        if let Err(e) = validate(&tokens) {
            println!("{e}");
            continue;
        }

        tokens = shunting_yard(tokens);
        match evaluate(tokens, &mut context) {
            Ok(result) => match result {
                EvalOutput::Assignment(var, val) => println!("Assigned value, {val}, to variable {var}"),
                EvalOutput::Value(value) => println!("{value}"),
            },
            Err(e) => println!("{e}"),
        }
    }
}