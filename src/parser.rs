use std::fmt::Display;
use crate::tokens::{BinaryOp, Function, Glyph, Token, UnaryOp, Value};

#[derive(Debug)]
pub struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.0)
    }
}


pub trait Ordering {
    fn can_precede(&self, other: &Token) -> bool;
}

impl Ordering for Token {
    fn can_precede(&self, other: &Token) -> bool {
        match self {
            Token::Func(function) => function.can_precede(other),
            Token::Val(value) => value.can_precede(other),
            Token::Glyph(glyph) => glyph.can_precede(other),
            Self::End => false,
            Self::Start => match other {
                Token::Val(_) => true,
                Token::Func(Function::UnaryOp(_)) => true,
                Token::Func(Function::NamedFunc(_)) => true,
                Token::Glyph(Glyph::LBracket) => true,
                _ => false, 
            }
        }
    }
}

impl Ordering for Function {
    fn can_precede(&self, other: &Token) -> bool {
        match self {
            Self::BinaryOp(op) => op.can_precede(other),
            Self::UnaryOp(op) => op.can_precede(other),
            Self::NamedFunc(_) => match other {
                Token::Glyph(Glyph::LBracket) => true,
                _ => false,
            }
        }
    }
}

impl Ordering for BinaryOp {
    fn can_precede(&self, other: &Token) -> bool {
        match other {
            Token::Val(_) => true,
            Token::Func(Function::UnaryOp(_)) => true,
            Token::Func(Function::NamedFunc(_)) => true,
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
        }
    }
}

impl Ordering for UnaryOp {
    fn can_precede(&self, other: &Token) -> bool {
        match other {
            Token::Val(_) => true,
            Token::Func(Function::UnaryOp(_)) => true,
            Token::Func(Function::NamedFunc(_)) => true,
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
        }
    }
}

impl Ordering for Value {
    fn can_precede(&self, other: &Token) -> bool {
        match other {
            Token::Func(Function::BinaryOp(_)) => true,
            Token::Glyph(Glyph::Comma) => true,
            Token::Glyph(Glyph::RBracket) => true,
            Token::End => true,
            _ => false,
        }
    }
}

impl Ordering for Glyph {
    fn can_precede(&self, other: &Token) -> bool {
        match self {
            Glyph::LBracket | Glyph::Comma => {
                match other {
                    Token::Func(Function::UnaryOp(_)) => true,
                    Token::Func(Function::NamedFunc(_)) => true,
                    Token::Val(_) => true,
                    Token::Glyph(Glyph::LBracket) => true,
                    _ => false,
                }
            }
            Glyph::RBracket => {
                match other {
                    Token::Func(Function::BinaryOp(_)) => true,
                    Token::Glyph(Glyph::Comma) => true,
                    Token::Glyph(Glyph::RBracket) => true,
                    Token::End => true,
                    _ => false,
                }
            }
        }
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


pub fn validate(tokens: &Vec<Token>) -> bool {
    if !validate_brackets(tokens) {
        return false;
    }
    
    for token in tokens.windows(2) {
        if !token[0].can_precede(&token[1]) {
            println!("{} cannot precede {}", token[0], token[1]);
            return  false;
        }
    }

    true
}

#[test]
fn test_validate_0() {
    let input = vec![
        Token::Start,
        Value::Var("a".to_string()).into(),
        BinaryOp::Add.into(),
        Value::Var("b".to_string()).into(),
        BinaryOp::Mul.into(),
        Function::NamedFunc("sin".to_string()).into(),
        Glyph::LBracket.into(),
        Value::Var("x".to_string()).into(),
        Glyph::RBracket.into(),
        Token::End,
    ];
    assert!(validate(&input) == true, "a + b * sin(x) was validated to false, expected: true.");
}

#[test]
fn test_validate_1() {
    let input = vec![
        Token::Start,
        Value::Scalar(11.0).into(),
        BinaryOp::Sub.into(),
        Value::Scalar(2.0).into(),
        BinaryOp::Div.into(),
        UnaryOp::Neg.into(),
        Value::Var("e".to_string()).into(),
        Token::End
    ];
    assert!(validate(&input) == true, "11-2/-e was validated to false, expected: true.");
}

#[test]
fn test_validate_2() {
    use crate::tokens::ExpressionBuilder;

    let mut exp_builder = ExpressionBuilder{vec: Vec::new()};
    exp_builder = exp_builder
        .start()
        .lbracket()
        .lbracket()
        .var("y")
        .rbracket()
        .end();

    assert!(validate(&exp_builder.vec) == false, "((y) was validated to true, expected: false.");
}