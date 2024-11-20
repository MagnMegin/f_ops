use std::{fmt::Display, process::Output};
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
    let mut index = 0;
    
    while let Some(t) = tokens.get(index) {
        if let Token::Name(name) = t {
            if tokens.get(index + 1) == Some(&Token::LBracket) {
                tokens[index] = Token::Func(name.to_string());
            }
            else {
                tokens[index] = Token::Var(name.to_string());
            }
        }

        index += 1;
    }
}

fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    todo!()
}

#[test]
fn test_parse_names() {
    let mut input = vec![
        Token::Name("exp".to_string()),
        Token::LBracket,
        Token::Name("x".to_string()),
        Token::Add,
        Token::Constant(1.0),
        Token::RBracket,
        Token::Mul,
        Token::Name("y".to_string())
        ];
    
    let output = vec![
        Token::Func("exp".to_string()),
        Token::LBracket,
        Token::Var("x".to_string()),
        Token::Add,
        Token::Constant(1.0),
        Token::RBracket,
        Token::Mul,
        Token::Var("y".to_string())
        ];
    parse_names(&mut input);

    assert!(input == output)
}