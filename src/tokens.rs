use std::{fmt::Display, str::Chars};
use core::iter::Peekable;

macro_rules! symbols {
    () => {
        '+' | '-' | '*' | '/' | '(' |
        ')'
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
pub struct InvalidTokenError(String);

impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid token error: {}", self.0)
    }
}


#[derive(Debug)]
pub enum Token {
    Constant(f32),
    Name(String),
    Add,
    Sub,
    Mul,
    Div,
    LBracket,
    RBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(x) => write!(f, "Constant({})", x),
            Self::Name(s) => write!(f, "Name({})", s),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::Mul => write!(f, "Mul"),
            Self::Div => write!(f, "Div"),
            Self::LBracket => write!(f, "LBracket"),
            Self::RBracket => write!(f, "RBracket"),

        }
    }
}


trait Lexer<'a> {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, InvalidTokenError>;
}


struct SymbolLexer;

impl <'a> Lexer<'a> for SymbolLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, InvalidTokenError> {
        let c = chars.next().ok_or(InvalidTokenError(String::from("Empty token")))?;
        
        match c {
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Sub),
            '*' => Ok(Token::Mul),
            '/' => Ok(Token::Div),
            '(' => Ok(Token::LBracket),
            ')' => Ok(Token::RBracket),
            _ => Err(InvalidTokenError(String::from(c)))
        }
    }
}


struct NumberLexer;

impl <'a> Lexer<'a> for NumberLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, InvalidTokenError> {
        if chars.peek().is_none() {
            return  Err(InvalidTokenError(String::from("Emtpy token")));
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
                        return Ok(Token::Constant(buffer.parse().unwrap()))
                    }
                    else {
                        buffer.push(c.clone());
                        has_dot = true;
                        chars.next();
                    }
                },
                _ => {
                    return Ok(Token::Constant(buffer.parse().unwrap()))
                },
            };
        }

        Ok(Token::Constant(buffer.parse().unwrap()))
    }
}


struct NameLexer;

impl <'a> Lexer<'a> for NameLexer {
    fn read_token(self, chars: &mut Peekable<Chars<'a>>) -> Result<Token, InvalidTokenError> {
        if chars.peek().is_none() {
            return Err(InvalidTokenError(String::from("Empty token")));
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


pub fn tokenize<'a>(s: &'a str) -> Result<Vec<Token>, InvalidTokenError> {
    let mut chars = s.chars().peekable();
    let mut tokens = Vec::new();


    while let Some(c) = chars.peek() {
        let t = match c {
            symbols!() => SymbolLexer.read_token(&mut chars),
            digits!() => NumberLexer.read_token(&mut chars),
            letters!() => NameLexer.read_token(&mut chars),
            _ => Err(InvalidTokenError(String::from(c.clone()))),
        };

        match t {
            Ok(t) => tokens.push(t),
            Err(e) => return Err(e),
        };

    };

    Ok(tokens)
}