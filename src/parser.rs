use std::fmt::Display;
use crate::tokens::Token;


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
        match t {
            Token::LBracket => l_bracket_count += 1,
            Token::RBracket => r_bracket_count += 1,
            _ => continue,
        }
    }

    l_bracket_count == r_bracket_count
}

// Converts Name tokens to Func and Var tokens
fn convert_names(tokens: &mut Vec<Token>) {
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

// Converts Sub token to Neg token if Sub represents a unary "-" operator
// (e.g. 1 * -1) "-" is unary in this context
fn convert_unary_sub(tokens: &mut Vec<Token>) {
    let mut index = 0;
    
    while let Some(t) = tokens.get(index) {
        if t == &Token::Sub {
            if let Some(t_prev) = tokens.get(index - 1) {
                match t_prev {
                    Token::Add | Token::Sub | Token::Mul | Token::Div | Token::Pow => tokens[index] = Token::Neg,
                    _ => (),
                }
            }
            else {
                tokens[index] = Token::Neg;
            }
        }

        index += 1;
    }    
}


fn to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    todo!()
}


#[test]
fn test_convert_names() {
    let mut input = vec![
        Token::Name("exp".to_string()),
        Token::LBracket,
        Token::Name("x".to_string()),
        Token::Add,
        Token::Const(1.0),
        Token::RBracket,
        Token::Mul,
        Token::Name("y".to_string())
        ];
    
    let output = vec![
        Token::Func("exp".to_string()),
        Token::LBracket,
        Token::Var("x".to_string()),
        Token::Add,
        Token::Const(1.0),
        Token::RBracket,
        Token::Mul,
        Token::Var("y".to_string())
        ];

    convert_names(&mut input);
    assert!(input == output)
}

#[test]
fn test_convert_unary_sub() {
    let mut input = vec![
        Token::Name("x".to_string()),
        Token::Mul,
        Token::Sub,
        Token::Const(1.0),
        ];

    let output = vec![
        Token::Name("x".to_string()),
        Token::Mul,
        Token::Neg,
        Token::Const(1.0),
        ];

    convert_unary_sub(&mut input);
    assert!(input == output)
}