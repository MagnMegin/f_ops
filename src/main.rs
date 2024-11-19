use std::io;
use tokenizer::tokenize;
use evaluator::evaluate;

pub mod tokens;
pub mod tokenizer;
pub mod parser;
pub mod evaluator;

fn main() {

    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.to_lowercase();
        
        let tokens = tokenize(&buffer.trim());

        match tokens {
            Ok(t) => {
                match evaluate(t) {
                    Ok(x) => println!("{:?}", x),
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e) => println!("{}", e),
        };
    }
}
