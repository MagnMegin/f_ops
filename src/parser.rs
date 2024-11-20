use std::fmt::Display;
use crate::tokens::{Token, Glyph};


#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}


fn validate_brackets(tokens: &Vec<Token>) -> bool {
    let mut l_bracket_count = 0;
    let mut r_bracket_count = 0;
    
    for t in tokens {
        if let Token::Glyph(glyph) = t {
            match glyph {
                Glyph::LBracket => l_bracket_count += 1,
                Glyph::RBracket => r_bracket_count += 1,
                _ => continue,
            }
        }

    }

    l_bracket_count == r_bracket_count
}

fn validate_operations(tokens: &Vec<Token>) -> bool {
    
    todo!()
}