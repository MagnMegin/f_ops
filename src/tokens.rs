use std::fmt::Display;

/// Shortcut for creating tokens. Is not constant.
#[allow(non_snake_case)]
pub fn TOKEN(sub_token: impl Into<Token>) -> Token {
    sub_token.into()
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

impl Token {
    pub fn variant_id(&self) -> i32 {
        match self {
            Token::Func(function) => {
                match function {
                    Function::Add => 0,
                    Function::Sub => 1,
                    Function::Mul => 2,
                    Function::Div => 3,
                    Function::Pow => 4,
                    Function::Neg => 5,
                    Function::NamedFunc(_) => 6,
                }
            },
            Token::Val(value) => {
                match value {
                    Value::Const(_) => 7,
                    Value::Var(_) => 8,
                }

            },
            Token::Glyph(glyph) => {
                match glyph {
                    Glyph::LBracket => 9,
                    Glyph::RBracket => 10,
                    Glyph::Comma => 11,
                }
            },
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    // Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    // Unary operations
    Neg,
    // Named functions (like sin, cos, exp or custom functions)
    NamedFunc(String),
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::Mul => write!(f, "Mul"),
            Self::Div => write!(f, "Div"),
            Self::Pow => write!(f, "Pow"),
            Self::Neg => write!(f, "Neg"),
            Self::NamedFunc(name) => write!(f, "NamedFunc({})", name) 
        }
    }
}

impl Into<Token> for Function {
    fn into(self) -> Token {
        Token::Func(self)
    }
}

impl Function {
    pub const fn presedence(&self) -> i32 {
        match self {
            Self::Add => 0,
            Self::Sub => 0,
            Self::Mul => 1,
            Self::Div => 1,
            Self::Pow => 2,
            Self::Neg => 1,
            Self::NamedFunc(_) => 3,
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