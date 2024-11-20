use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Const(f32),
    Var(String),
    Func(String),
    Add,
    Sub,
    Neg,
    Mul,
    Div,
    Pow,
    LBracket,
    RBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Const(x) => write!(f, "Constant({})", x),
            Self::Var(s) => write!(f, "Var({})", s),
            Self::Func(s) => write!(f, "Func({})", s),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::Neg => write!(f, "Neg"),
            Self::Mul => write!(f, "Mul"),
            Self::Div => write!(f, "Div"),
            Self::Pow => write!(f, "Pow"),
            Self::LBracket => write!(f, "LBracket"),
            Self::RBracket => write!(f, "RBracket"),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum NewToken {
    Function(Function),
    Value(Value),
    Symbol(Symbol),
}

impl Display for NewToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(function) => function.fmt(f),
            Self::Value(value) => value.fmt(f),
            Self::Symbol(symbol) => symbol.fmt(f),
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


#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LBracket,
    RBracket,
    Comma,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LBracket => write!(f, "LBracket"),
            Self::RBracket => write!(f, "RBracket"),
            Self::Comma => write!(f, "Comma"),
        }
    }
}