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



pub fn evaluate(postfix_tokens: Vec<Token>, context: &Context) -> Result<f32, EvalError> {
    let mut eval_stack = Vec::new();

    for token in postfix_tokens {
        match token {
            Token::Val(v) => match v {
                Value::Scalar(x) => eval_stack.push(x),
                Value::Var(name) => match context.var(&name) {
                    Some(x) => eval_stack.push(x),
                    None => return Err(EvalError::UndefinedVariable(name.clone())),
                },
            }
            Token::Func(function) => match function {
                Function::Assign => return Err(EvalError::NotImplemented(Function::Assign.into())),
                Function::BinaryOp(op) => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match op {
                        BinaryOp::Add => eval_stack.push(n2 + n1),
                        BinaryOp::Sub => eval_stack.push(n2 - n1),
                        BinaryOp::Mul => eval_stack.push(n2 * n1),
                        BinaryOp::Div => eval_stack.push(n2 / n1),
                        BinaryOp::Pow => eval_stack.push(n2.powf(n1)),
                    }
                }
                Function::UnaryOp(op) => {
                    let n = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match op {
                        UnaryOp::Neg => eval_stack.push(-1.0 * n),
                    }
                }
                Function::NamedFunc(name) => {
                    let n = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    match context.call_func(&name, n) {
                        Some(val) => eval_stack.push(val),
                        None => return Err(EvalError::UndfinedFunction(name.clone())),
                    }
                }
            }
            _ => return Err(EvalError::InvalidToken),
        }
    }

    return Ok(eval_stack.pop().ok_or(EvalError::MissingResult)?);
}