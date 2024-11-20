use std::fmt::Display;
use crate::tokens::Token;


#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}

fn verify_brackets(tokens: &Vec<Token>) -> bool {
    let mut l_bracket_count = 0;
    let mut r_bracket_count = 0;
    
    for t in tokens {
        match t {
            Token::LBracket => l_bracket_count += 1,
            Token::RBracket => r_bracket_count += 1,
            _ => continue,
        }
    }

    l_bracket_count == r_bracket_count
}

fn parse_names(tokens: &mut Vec<Token>) {
    
}

fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    todo!()
}