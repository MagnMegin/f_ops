use std::fmt::Display;
use crate::tokens::{Function, Token, BinaryOp, UnaryOp, Value, Glyph};

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
        match other {
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
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


fn validate(tokens: &Vec<Token>) -> bool {
    if !validate_brackets(tokens) {
        return false;
    }
    
    for t in tokens.windows(2) {
        t[0].can_precede(&t[1]);
    }

    todo!()
}