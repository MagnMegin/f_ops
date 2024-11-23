use std::fmt::Display;

use crate::{app_context::Context, tokens::{BinaryOp, Function, Token, UnaryOp, Value}};

#[derive(Debug)]
pub enum EvalError {
    MissingArgument,
    InvalidToken,
    MissingResult,
    NotImplemented(Token),
    IncorrectAssignment(Token),
    UndefinedVariable(String),
    UndfinedFunction(String),
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvaluatorError -> ")?;
        match self {
            Self::MissingArgument => write!(f, "Missing Argument"),
            Self::InvalidToken => write!(f, "Invalid Token"),
            Self::MissingResult => write!(f, "Missing Result"),
            Self::NotImplemented(token) => write!(f, "Not Implemented: {token}"),
            Self::IncorrectAssignment(token) => write!(f, "Cannot assign to {token}"),
            Self::UndefinedVariable(name) => write!(f, "Undefined Variable: '{name}'"),
            Self::UndfinedFunction(name) => write!(f, "Undefined Function: '{name}'"),
        }
    }
}


#[derive(Debug)]
pub enum EvalOutput {
    Value(f32),
    Assignment(String, f32),
}


impl Value {
    pub fn collect(self, context: &Context) -> Result<f32, EvalError> {
        match self {
            Value::Scalar(x) => Ok(x),
            Value::Var(name) => match context.var(&name) {
                Some(x) => Ok(x),
                None => Err(EvalError::UndefinedVariable(name.clone())),
            },
        }
    }

    pub fn add(self, other: Value, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(self.collect(context)? + other.collect(context)?))
    }

    pub fn sub(self, other: Value, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(self.collect(context)? - other.collect(context)?))
    }

    pub fn mul(self, other: Value, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(self.collect(context)? * other.collect(context)?))
    }

    pub fn div(self, other: Value, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(self.collect(context)? / other.collect(context)?))
    }

    pub fn pow(self, other: Value, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(self.collect(context)?.powf(other.collect(context)?)))
    }

    pub fn neg(self, context: &Context) -> Result<Value, EvalError> {
        Ok(Value::Scalar(-1.0 * self.collect(context)?))
    }
}


pub fn evaluate(postfix_tokens: Vec<Token>, context: &mut Context) -> Result<EvalOutput, EvalError>{
    let mut eval_stack = Vec::new();

    for token in postfix_tokens {
        match token {
            Token::Val(value) => eval_stack.push(value),
            Token::Func(function) => match function {
                Function::Assign => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    if let Value::Var(name) = n2 {
                        let value = n1.collect(&context)?;
                        context.set_var(&name, value);
                        return Ok(EvalOutput::Assignment(name.clone(), value));
                    }
                }
                Function::BinaryOp(op) => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match op {
                        BinaryOp::Add => eval_stack.push(n2.add(n1, &context)?),
                        BinaryOp::Sub => eval_stack.push(n2.sub(n1, &context)?),
                        BinaryOp::Mul => eval_stack.push(n2.mul(n1, &context)?),
                        BinaryOp::Div => eval_stack.push(n2.div(n1, &context)?),
                        BinaryOp::Pow => eval_stack.push(n2.pow(n1, &context)?),
                    }
                }
                Function::UnaryOp(op) => {
                    let n = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match op {
                        UnaryOp::Neg => eval_stack.push(n.neg(&context)?),
                    }
                }
                Function::NamedFunc(name) => {
                    let n = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match context.call_func(&name, n.collect(&context)?) {
                        Some(n) => eval_stack.push(Value::Scalar(n)),
                        None => return Err(EvalError::UndfinedFunction(name.clone())),
                    }
                }
            }
            _ => return Err(EvalError::InvalidToken),
        }
    }

    if let Some(val) = eval_stack.pop() {
        Ok(EvalOutput::Value(val.collect(&context)?))
    }
    else {
        Err(EvalError::MissingResult)
    }
}