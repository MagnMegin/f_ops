use std::io;
use tokenizer::tokenize;

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;

fn main() {
    println!("{:?}", tokenize("a a a 2").unwrap())
}