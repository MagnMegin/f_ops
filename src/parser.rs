use std::fmt::Display;
use crate::tokens::Token;


#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}

fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    todo!()
}