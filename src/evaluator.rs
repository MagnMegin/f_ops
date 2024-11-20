use crate::tokens::{Function, Token, Value};

#[derive(Debug)]
pub enum EvalError {
    MissingArgument,
    MissingResult,
    NotImplemented,
}

pub fn evaluate(postfix_tokens: Vec<Token>) -> Result<f32, EvalError> {
    let mut eval_stack = Vec::new();

    for token in postfix_tokens {
        match token {
            Token::Val(v) => match v {
                Value::Const(x) => eval_stack.push(x),
                Value::Var(_) => return Err(EvalError::NotImplemented),
            }
            Token::Func(f) => match f {
                Function::Add => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    eval_stack.push(n2 + n1);
                },
                Function::Sub => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    eval_stack.push(n2 - n1);
                },
                Function::Mul => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    eval_stack.push(n2 * n1);
                },
                Function::Div => {
                    let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                    eval_stack.push(n2 / n1);
                },
                _ => {
                    return Err(EvalError::NotImplemented);
                }
            }
            _ => {
                return Err(EvalError::NotImplemented);
            } 
        }
    }

    return Ok(eval_stack.pop().ok_or(EvalError::MissingResult)?);
}