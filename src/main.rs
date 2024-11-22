use tokenizer::tokenize;
use parser::validate;

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;

fn main() {
    validate(&tokenize("a + b * sin(x)/5").unwrap());
}