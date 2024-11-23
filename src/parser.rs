use std::fmt::Display;
use crate::tokens::{BinaryOp, Function, Glyph, Token, UnaryOp, Value};

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    UnevenBrackets,
    IncorrectAssign,
    OrderError(Token, Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError -> ")?;
        match self {
            Self::UnevenBrackets => write!(f, "Uneven Brackets"),
            Self::IncorrectAssign => write!(f, "Incorrect Assign"),
            Self::OrderError(t1, t2) => write!(f, "{t1} cannot precede {t2}"),
        }
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
            Self::Assign => match other {
                Token::Val(_) => true,
                Token::Func(Function::UnaryOp(_)) => true,
                Token::Func(Function::NamedFunc(_)) => true,
                Token::Glyph(Glyph::LBracket) => true,
                _ => false,                
            }
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
            Token::Func(Function::Assign) => true,
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



fn validate_brackets(tokens: &Vec<Token>) -> Result<(), ParserError> {
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

    if l_bracket_count == r_bracket_count {
        return Ok(());
    }
    Err(ParserError::UnevenBrackets)
}


pub fn validate(tokens: &Vec<Token>) -> Result<(), ParserError> {
    validate_brackets(tokens)?;
    
    for (count, token) in tokens.windows(2).enumerate() {
        if !token[0].can_precede(&token[1]) {
            return Err(ParserError::OrderError(token[0].clone(), token[1].clone()));
        }
        if token[0] == Token::Func(Function::Assign) && count != 2 {
            return Err(ParserError::IncorrectAssign);
        }
    };

    Ok(())
}

// Converts infix to postfix. Needs to be done before evaluation.
pub fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operations: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Val(_) => output.push(token),
            Token::Func(Function::NamedFunc(_)) => operations.push(token),
            Token::Func(ref function) => 
            loop {
                match operations.last() {
                    None | Some(Token::Glyph(_)) => {
                        operations.push(token);
                        break;
                    }
                    Some(Token::Func(prev_function)) if prev_function.presedence() <= function.presedence() => {
                        operations.push(token);
                        break;
                    }
                    Some(Token::Func(_)) => {
                        let prev_function = operations.pop().unwrap();
                        output.push(prev_function);
                    }
                    Some(_) => (),                   
                }

            },
            Token::Glyph(Glyph::LBracket) => operations.push(token),
            Token::Glyph(Glyph::RBracket) => 
            while let Some(prev_token) = operations.last() {
                if let Token::Glyph(Glyph::LBracket) = prev_token{
                    operations.pop().unwrap();
                    break;
                }
                let prev_token = operations.pop().unwrap();
                output.push(prev_token);
            }
            Token::Glyph(Glyph::Comma) => 
            while let Some(Token::Func(_)) = operations.last() {
                let prev_function = operations.pop().unwrap();
                output.push(prev_function);
            }
            Token::Start | Token::End => continue,
        }
    }

    for op in operations.into_iter().rev() {
        output.push(op);
    }
    
    output
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
    assert!(validate(&input).is_ok(), "a + b * sin(x) was validated to false, expected: true.");
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
    assert!(validate(&input).is_ok(), "11-2/-e was validated to false, expected: true.");
}

#[test]
fn test_validate_2() {
    use crate::tokens::ExpressionBuilder;

    let exp_builder = ExpressionBuilder::new();
    let vec = exp_builder
        .start()
        .lbracket()
        .lbracket()
        .var("y")
        .rbracket()
        .end()
        .collect();

    assert!(validate(&vec).is_err(), "((y) was validated to true, expected: false.");
}

#[test]
fn tes_shunting_0() {
    use crate::tokens::ExpressionBuilder;
    let exp_builder = ExpressionBuilder::new();
    let input = exp_builder
        .start()
        .scalar(1.0)
        .add()
        .func("sin")
        .lbracket()
        .scalar(3.14)
        .mul()
        .scalar(2.0)
        .rbracket()
        .end()
        .collect();

    let exp_builder = ExpressionBuilder::new();
    let output = exp_builder
        .scalar(1.0)
        .scalar(3.14)
        .scalar(2.0)
        .mul()
        .func("sin")
        .add()
        .collect();

    
    assert!(shunting_yard(input) == output)
}