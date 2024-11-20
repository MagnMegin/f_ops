use crate::tokens::Token;

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
            Token::Const(x) => eval_stack.push(x),
            Token::Add => {
                let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                eval_stack.push(n2 + n1);
            },
            Token::Sub => {
                let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                eval_stack.push(n2 - n1);
            },
            Token::Mul => {
                let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                eval_stack.push(n2 * n1);
            },
            Token::Div => {
                let n1 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                let n2 = eval_stack.pop().ok_or(EvalError::MissingArgument)?;
                eval_stack.push(n2 / n1);
            },
            _ => {
                return Err(EvalError::NotImplemented);
            }
        }
    }

    return Ok(eval_stack.pop().ok_or(EvalError::MissingResult)?);
}