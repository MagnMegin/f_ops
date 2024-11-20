use std::fmt::Display;
use crate::tokens::{Token, Glyph};


#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}


// Converts Sub token to Neg token if Sub represents a unary "-" operator
// (e.g. 1 * -1) "-" is unary in this context
// fn convert_unary_sub(tokens: &mut Vec<Token>) {
//     let mut index = 0;
    
//     while let Some(t) = tokens.get(index) {
//         if t == &Token::Sub {
//             if let Some(t_prev) = tokens.get(index - 1) {
//                 match t_prev {
//                     Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Pow => tokens[index] = Token::Neg,
//                     _ => (),
//                 }
//             }
//             else {
//                 tokens[index] = Token::Neg;
//             }
//         }

//         index += 1;
//     }    
// }

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