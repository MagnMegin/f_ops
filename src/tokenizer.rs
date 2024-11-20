use std::{fmt::Display, str::Chars};
use crate::tokens::{Function, Glyph, Token, Value, TOKEN};

macro_rules! symbols {
    () => {
        '+' | '-' | '*' | '/' | '^' |
        '(' | ')' | ','
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


pub struct LexingReader<'a> {
    iterator: Chars<'a>,
    prev_char: Option<char>, 
    current_char: Option<char>,
    next_char: Option<char>,
}

impl <'a> LexingReader<'a> {
    pub fn new(string: &'a str) -> Self {
        let mut iterator = string.chars();
        let prev_char = None;
        let current_char = iterator.next();
        let next_char = iterator.next();

        Self {iterator, prev_char, current_char, next_char}
    }

    pub fn prev_char(&self) -> Option<char> {
        self.prev_char
    }

    pub fn current_char(&self) -> Option<char> {
        self.current_char
    }

    pub fn next_char(&self) -> Option<char> {
        self.next_char
    }

    pub fn advance(&mut self) {
        self.prev_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.iterator.next();
    }

    pub fn finished(&self) -> bool {
        self.current_char.is_none()
    }
}


trait Lexer<'a> {
    fn read_token(self, reader: &mut LexingReader) -> Result<Token, TokenizerError>;
}


struct SymbolLexer;

impl <'a> Lexer<'a> for SymbolLexer {
    fn read_token(self, reader: &mut LexingReader) -> Result<Token, TokenizerError> {
        assert!(!reader.finished(), "Cannot tokenize: empty reader");

        let c = reader.current_char().ok_or(TokenizerError::EmptyToken)?;
        let result = match c {
            '+' => Ok(Token::Func(Function::Add)),
            '-' => {
                match reader.prev_char {
                    None | Some( '+' | '-' | '*' | '/' | '^' | ',' ) => Ok(Token::Func(Function::Neg)),
                    _ => Ok(Token::Func(Function::Sub)),
                }
            }
            '*' => Ok(Token::Func(Function::Mul)),
            '/' => Ok(Token::Func(Function::Div)),
            '^' => Ok(Token::Func(Function::Pow)),
            '(' => Ok(Token::Glyph(Glyph::LBracket)),
            ')' => Ok(Token::Glyph(Glyph::RBracket)),
            ',' => Ok(Token::Glyph(Glyph::Comma)),
            _ => Err(TokenizerError::IncorrectCharacter(String::from(c)))
        };
        
        reader.advance();
        result
    }
}


struct NumberLexer;

impl <'a> Lexer<'a> for NumberLexer {
    fn read_token(self, reader: &mut LexingReader) -> Result<Token, TokenizerError> {       
        assert!(!reader.finished(), "Cannot tokenize: empty reader");
        
        let mut buffer = String::new();
        let mut has_dot = false; 
        while let Some(c) = reader.current_char() {
            if c.is_numeric() {
                buffer.push(c);
                reader.advance();
            }
            else if c == '.' {
                if has_dot {
                    return Ok(Token::Val(Value::Const(buffer.parse().unwrap())));
                }
                else {
                    buffer.push(c);
                    has_dot = true;
                    reader.advance();
                }
                
            }
            else {
                return Ok(Token::Val(Value::Const(buffer.parse().unwrap())));
            };
        }

        Ok(Token::Val(Value::Const(buffer.parse().unwrap())))
    }
}


struct CharacterLexer;

impl <'a> Lexer<'a> for CharacterLexer {
    fn read_token(self, reader: &mut LexingReader) -> Result<Token, TokenizerError> {
        assert!(!reader.finished(), "Cannot tokenize: empty reader");
        
        let mut buffer = String::new();
        while let Some(c) = reader.current_char() {
            if c.is_alphabetic() {
                buffer.push(c);
                reader.advance();
            }
            else if c == '(' {
                return Ok(Token::Func(Function::NamedFunc(buffer)));
            }
            else {
                return Ok(Token::Val(Value::Var(buffer)));
            }
        };

        Ok(Token::Val(Value::Var(buffer)))
    }
}


pub fn tokenize<'a>(s: &'a str) -> Result<Vec<Token>, TokenizerError> { 
    let mut reader = LexingReader::new(s);
    let mut tokens = Vec::new();

    while let Some(c) = reader.current_char() {
        let token: Token;
        
        if let symbols!() = c {
            token = SymbolLexer.read_token(&mut reader)?;
        }
        else if c.is_numeric() {
            token = NumberLexer.read_token(&mut reader)?;
        }
        else if c.is_alphabetic() {
            token = CharacterLexer.read_token(&mut reader)?;
        }
        else if ' ' == c {
            reader.advance();
            continue;
        }
        else {
            return Err(TokenizerError::IncorrectCharacter(String::from(c)));
        }

        tokens.push(token);
    };

    Ok(tokens)
}


#[test]
fn test_tokenize() {
    let input = "+";
    let output = vec![TOKEN(Function::Add)];
    assert!(tokenize(input).unwrap() == output, "Single add failed");

    let input = ",+3 *sin(x)";
    let output: Vec<Token> = vec![
        TOKEN(Glyph::Comma),
        TOKEN(Function::Add),
        TOKEN(Value::Const(3.0)),
        TOKEN(Function::Mul),
        TOKEN(Function::NamedFunc("sin".to_string())),
        TOKEN(Glyph::LBracket),
        TOKEN(Value::Var("x".to_string())),
        TOKEN(Glyph::RBracket),
    ];
    assert!(tokenize(input).unwrap() == output, ",+3 *sin(x) Failed");
}