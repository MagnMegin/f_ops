use std::{fmt::Display, str::Chars};
use crate::tokens::{BinaryOp, Function, Glyph, Token, UnaryOp, Value};

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
            '+' => Ok(BinaryOp::Add.into()),
            '-' => {
                // This is done to split between binary - and unary -.
                match reader.prev_char {
                    None | Some( '+' | '-' | '*' | '/' | '^' | ',' ) => Ok(UnaryOp::Neg.into()),
                    _ => Ok(BinaryOp::Sub.into()),
                }
            }
            '*' => Ok(BinaryOp::Mul.into()),
            '/' => Ok(BinaryOp::Div.into()),
            '^' => Ok(BinaryOp::Pow.into()),
            '(' => Ok(Glyph::LBracket.into()),
            ')' => Ok(Glyph::RBracket.into()),
            ',' => Ok(Glyph::Comma.into()),
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
                    return Ok(Value::Const(buffer.parse().unwrap()).into());
                }
                else {
                    buffer.push(c);
                    has_dot = true;
                    reader.advance();
                }
                
            }
            else {
                return Ok(Value::Const(buffer.parse().unwrap()).into());
            };
        }

        Ok(Value::Const(buffer.parse().unwrap()).into())
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
                return Ok(Function::NamedFunc(buffer).into());
            }
            else {
                return Ok(Value::Var(buffer).into());
            }
        };

        Ok(Value::Var(buffer).into())
    }
}


pub fn tokenize<'a>(s: &'a str) -> Result<Vec<Token>, TokenizerError> { 
    let mut reader = LexingReader::new(s);
    let mut tokens = Vec::with_capacity(s.len() + 2);
    tokens.push(Token::Start);

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
    
    tokens.push(Token::End);
    Ok(tokens)
}


#[test]
fn test_tokenize() {
    let input = "+";
    let output = vec![BinaryOp::Add.into()];
    assert!(tokenize(input).unwrap() == output, "Single add failed");

    let input = ",+3 *sin(x)";
    let output: Vec<Token> = vec![
        Glyph::Comma.into(),
        BinaryOp::Add.into(),
        Value::Const(3.0).into(),
        BinaryOp::Mul.into(),
        Function::NamedFunc("sin".to_string()).into(),
        Glyph::LBracket.into(),
        Value::Var("x".to_string()).into(),
        Glyph::RBracket.into(),
    ];
    assert!(tokenize(input).unwrap() == output, ",+3 *sin(x) Failed");
}