use std::{fmt::Display, str::Chars};
use core::iter::Peekable;
use crate::tokens::Token;

macro_rules! symbols {
    () => {
        '+' | '-' | '*' | '/' | '^' |
        '(' | ')'
    };
}

macro_rules! digits {
    () => {
        '0' | '1' | '2' | '3' | '4' |
        '5' | '6' | '7' | '8' | '9'
    };
}

macro_rules! letters {
    () => {
        'a' | 'b' | 'c' | 'd' | 'e' |
        'f' | 'g' | 'h' | 'i' | 'j' |
        'k' | 'l' | 'm' | 'n' | 'o' |
        'p' | 'q' | 'r' | 's' | 't' |
        'u' | 'v' | 'w' | 'x' | 'y' |
        'z'
    };
}

#[derive(Debug)]
pub enum TokenizerError {
    EmptyToken,
    IncorrectCharacter(String),
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyToken => String::from("Empty token"),
            Self::IncorrectCharacter(c) => String::from("Incorrect character: ") + c,  
        };
        write!(f, "TokenizerError -> {}", message)
    }
}


trait Lexer<'a> {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, TokenizerError>;
}


struct SymbolLexer;

impl <'a> Lexer<'a> for SymbolLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, TokenizerError> {
        let c = chars.next().ok_or(TokenizerError::EmptyToken)?;
        
        match c {
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '(' => Ok(Token::LBracket),
            ')' => Ok(Token::RBracket),
            '^' => Ok(Token::Pow),
            _ => Err(TokenizerError::IncorrectCharacter(String::from(c)))
        }
    }
}


struct NumberLexer;

impl <'a> Lexer<'a> for NumberLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, TokenizerError> {
        if chars.peek().is_none() {
            return  Err(TokenizerError::EmptyToken);
        }
        
        let mut buffer = String::new();
        let mut has_dot = false;
        
        while let Some(c) = chars.peek() {
            match c {
                digits!() => {
                    buffer.push(c.clone());
                    chars.next();
                },
                '.' => {
                    if has_dot {
                        return Ok(Token::Const(buffer.parse().unwrap()))
                    }
                    else {
                        buffer.push(c.clone());
                        has_dot = true;
                        chars.next();
                    }
                },
                _ => {
                    return Ok(Token::Const(buffer.parse().unwrap()))
                },
            };
        }

        Ok(Token::Const(buffer.parse().unwrap()))
    }
}


struct NameLexer;

impl <'a> Lexer<'a> for NameLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, TokenizerError> {
        if chars.peek().is_none() {
            return Err(TokenizerError::EmptyToken);
        }
        
        let mut buffer = String::new();
        
        while let Some(c) = chars.peek() {
            match c {
                letters!() => {
                    buffer.push(c.clone());
                    chars.next();
                }
                _ =>{
                    return Ok(Token::Name(buffer));
                }
            }
        };

        Ok(Token::Name(buffer))
    }
}


pub fn tokenize<'a>(s: &'a str) -> Result<Vec<Token>, TokenizerError> {
    let mut chars = s.chars().peekable();
    let mut tokens = Vec::new();


    while let Some(c) = chars.peek() {
        let t = match c {
            symbols!() => SymbolLexer.read_token(&mut chars),
            digits!() => NumberLexer.read_token(&mut chars),
            letters!() => NameLexer.read_token(&mut chars),
            ' ' => {chars.next(); continue;},
            _ => Err(TokenizerError::IncorrectCharacter(String::from(c.clone()))),
        };

        match t {
            Ok(t) => tokens.push(t),
            Err(e) => return Err(e),
        };

    };

    Ok(tokens)
}