use std::io;

use tokens::tokenize;

pub mod tokens;
pub mod parser;
pub mod evaluator;

fn main() {

    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.replace(" ", "").to_lowercase();
        
        let tokens = tokenize(&buffer.trim());

        match tokens {
            Ok(t) => println!("{:?}", t),
            Err(e) => println!("{}", e),
        };
    }
}
