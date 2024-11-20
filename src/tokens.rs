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