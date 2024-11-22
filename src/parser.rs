use std::fmt::Display;
use crate::tokens::{Function, Glyph, Ordering, Token};

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


fn validate(tokens: &Vec<Token>) -> bool {
    if !validate_brackets(tokens) {
        return false;
    }
    
    for t in tokens.windows(2) {
        t[0].can_precede(&t[1]);
    }

    todo!()
}