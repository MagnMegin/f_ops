use std::fmt::Display;

/// Shortcut for creating tokens. Is not constant.
#[allow(non_snake_case)]
pub fn TOKEN(sub_token: impl Into<Token>) -> Token {
    sub_token.into()
}

pub trait Ordering {
    fn can_precede(&self, other: Token) -> bool;
}



#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Func(Function),
    Val(Value),
    Glyph(Glyph),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Func(function) => function.fmt(f),
            Self::Val(value) => value.fmt(f),
            Self::Glyph(glyph) => glyph.fmt(f),
        }
    }
}

impl Ordering for Token {
    fn can_precede(&self, other: Token) -> bool {
        match self {
            Token::Func(function) => function.can_precede(other),
            Token::Val(value) => value.can_precede(other),
            Token::Glyph(glyph) => glyph.can_precede(other),
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    NamedFunc(String),
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BinaryOp(op) => op.fmt(f),
            Self::UnaryOp(op) => op.fmt(f),
            Self::NamedFunc(name) => write!(f, "NamedFunc({})", name) 
        }
    }
}

impl Into<Token> for Function {
    fn into(self) -> Token {
        Token::Func(self)
    }
}

impl Ordering for Function {
    fn can_precede(&self, other: Token) -> bool {
        match other {
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
        }
    }
}

impl Function {
    pub const fn presedence(&self) -> i32 {
        match self {
            Self::BinaryOp(op) => {
                match op {
                    BinaryOp::Add => 0,
                    BinaryOp::Sub => 0,
                    BinaryOp::Mul => 1,
                    BinaryOp::Div => 1,
                    BinaryOp::Pow => 2,
                }
            }
            Self::UnaryOp(UnaryOp::Neg) => 1,
            Self::NamedFunc(_) => 3,
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::Mul => write!(f, "Mul"),
            Self::Div => write!(f, "Div"),
            Self::Pow => write!(f, "Pow"), 
        }
    }
}

impl Into<Function> for BinaryOp {
    fn into(self) -> Function {
        Function::BinaryOp(self)
    }
}

impl Into<Token> for BinaryOp {
    fn into(self) -> Token {
        Token::Func(Function::BinaryOp(self))
    }
}

impl Ordering for BinaryOp {
    fn can_precede(&self, other: Token) -> bool {
        match other {
            Token::Val(_) => true,
            Token::Func(Function::UnaryOp(_)) => true,
            Token::Func(Function::NamedFunc(_)) => true,
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg => write!(f, "Neg"),
        }
    }
}

impl Into<Function> for UnaryOp {
    fn into(self) -> Function {
        Function::UnaryOp(self)
    }
}

impl Into<Token> for UnaryOp {
    fn into(self) -> Token {
        Token::Func(Function::UnaryOp(self))
    }
}

impl Ordering for UnaryOp {
    fn can_precede(&self, other: Token) -> bool {
        match other {
            Token::Val(_) => true,
            Token::Func(Function::UnaryOp(_)) => true,
            Token::Func(Function::NamedFunc(_)) => true,
            Token::Glyph(Glyph::LBracket) => true,
            _ => false,
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Const(f32),
    Var(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(x) => write!(f, "Const({})", x),
            Self::Var(name) => write!(f, "Var({})", name),
        }
    }
}

impl Into<Token> for Value {
    fn into(self) -> Token {
        Token::Val(self)
    }
}

impl Ordering for Value {
    fn can_precede(&self, other: Token) -> bool {
        match other {
            Token::Func(Function::BinaryOp(_)) => true,
            Token::Glyph(Glyph::Comma) => true,
            Token::Glyph(Glyph::RBracket) => true,
            _ => false,
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Glyph {
    LBracket,
    RBracket,
    Comma,
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LBracket => write!(f, "LBracket"),
            Self::RBracket => write!(f, "RBracket"),
            Self::Comma => write!(f, "Comma"),
        }
    }
}

impl Into<Token> for Glyph {
    fn into(self) -> Token {
        Token::Glyph(self)
    }
}

impl Ordering for Glyph {
    fn can_precede(&self, other: Token) -> bool {
        match *self {
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
                    _ => false,
                }
            }
        }
    }
}