use std::fmt::Display;

use crate::tokens::Token;

#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}


pub struct TokenChain {
    tokens: Vec<Token> 
}

pub struct Validator;

impl Validator {
    fn validate(tokens: Vec<Token>) -> Option<TokenChain> {
        Some(TokenChain{tokens})
    }
}