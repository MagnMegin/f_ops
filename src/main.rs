use std::io;

use tokens::tokenize;

pub mod tokens;
pub mod parser;

fn main() {

    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        
        let tokens = tokenize(&buffer.trim());

        match tokens {
            Ok(t) => println!("{:?}", t),
            Err(e) => println!("{}", e),
        };
    }
}
