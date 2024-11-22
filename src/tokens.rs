use std::fmt::Display;


#[derive(Debug, Clone)]
pub struct ExpressionBuilder {
    pub vec: Vec<Token>
}

impl Display for ExpressionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let len = self.vec.len();
        for token in &self.vec[0..len-1] {
            write!(f, "{},", token)?;
        }

        if let Some(token) = self.vec.last() {
            write!(f, "{}", token)?;
        }

        write!(f, "]")?;
        
        Ok(())
    }
}

impl ExpressionBuilder {
    pub fn start(mut self) -> Self {
        self.vec.push(Token::Start);
        self
    }

    pub fn end(mut self) -> Self {
        self.vec.push(Token::End);
        self
    }

    pub fn binop(mut self, op: BinaryOp) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(op)));
        self
    }

    pub fn add(mut self) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(BinaryOp::Add)));
        self
    }

    pub fn sub(mut self) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(BinaryOp::Sub)));
        self
    }

    pub fn mul(mut self) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(BinaryOp::Mul)));
        self
    }

    pub fn div(mut self) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(BinaryOp::Div)));
        self
    }

    pub fn pow(mut self) -> Self {
        self.vec.push(Token::Func(Function::BinaryOp(BinaryOp::Pow)));
        self
    }

    pub fn neg(mut self) -> Self {
        self.vec.push(Token::Func(Function::UnaryOp(UnaryOp::Neg)));
        self
    }

    pub fn func(mut self, func: &str) -> Self {
        self.vec.push(Token::Func(Function::NamedFunc(func.to_string())));
        self
    }

    pub fn sclar(mut self, scalar: f32) -> Self {
        self.vec.push(Token::Val(Value::Scalar(scalar)));
        self
    }

    pub fn var(mut self, var: &str) -> Self {
        self.vec.push(Token::Val(Value::Var(var.to_string())));
        self
    }

    pub fn lbracket(mut self) -> Self {
        self.vec.push(Token::Glyph(Glyph::LBracket));
        self
    }

    pub fn rbracket(mut self) -> Self {
        self.vec.push(Token::Glyph(Glyph::RBracket));
        self
    }

    pub fn comma(mut self) -> Self {
        self.vec.push(Token::Glyph(Glyph::Comma));
        self
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Start,
    End,
    Func(Function),
    Val(Value),
    Glyph(Glyph),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::End => write!(f, "End"),
            Self::Func(function) => function.fmt(f),
            Self::Val(value) => value.fmt(f),
            Self::Glyph(glyph) => glyph.fmt(f),
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



#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Scalar(f32),
    Var(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scalar(x) => write!(f, "Const({})", x),
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