use std::io;

use app_context::Context;
use evaluator::evaluate;
use tokenizer::{tokenize, tokenize_unpadded};
use parser::validate;

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;
pub mod app_context;

fn main() {
    let mut context = Context::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();
        if input.chars().next() == Some('/') {
            input.remove(0);
            let mut input = input.split(" ");
            if input.next() == Some("set") {
                let varname = input.next().unwrap();
                let value = input.next().unwrap();
                println!("set {} to {}", varname, value);
                context.set_var(varname, value.parse().unwrap());
            }
        }
        else {
            let tokens = tokenize_unpadded(&input).unwrap();
            println!("{:?}", tokens);
            let result = evaluate(tokens, &context).unwrap();
            println!("{}", result)
        }

    }
}