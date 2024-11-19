use std::fmt::Display;
use crate::tokens::Token;



#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}

fn brackets_validated(tokens: &mut Vec<Token>) -> bool {
    
    todo!("Implement later")
}

fn simplify_subtraction(tokens: &mut Vec<Token>) {
    let mut prev_token = Token::Add; //Add is an operation so if the first token is Sub it will be converted to Neg

    // for i in 0..tokens.len() {
    //     if tokens[i] == Token::Sub {
    //         if prev_token == Token
    //     }

    //     prev_token = tokens[i].clone();

    // }
}

fn parse(tokens: &mut Vec<Token>) -> bool{
    if !brackets_validated(tokens) {
        return false;
    }
    return true;
}

struct Function {
    
}

struct Variable {

}

enum Expression {
    Const(f32),
    Var(String),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Func(String, Box<Expression>),
}

impl Expression {
    fn evaluate(&self) -> f32 {
        match self {
            Self::Const(x) => *x,
            Self::Var(s) => s.len() as f32,
            Self::Add(e1, e2) => e1.evaluate() + e2.evaluate(),
            Self::Mul(e1, e2) => e1.evaluate() + e2.evaluate(),
            Self::Func(_s, e) => e.evaluate(),
        }
    }
}