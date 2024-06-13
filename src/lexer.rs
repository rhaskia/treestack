use crate::error::{Error, Positioned};

pub struct Lexer {
    index: usize,
    program: Vec<char>,
    tokens: Vec<Positioned<Token>>,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        let program = program.chars().collect();
        Lexer { index: 0, program, tokens: Vec::new() }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        self.tokens.clear();

        while let Some(next_char) = self.next() {
            match next_char {
                ';' => self.push(Token::Semicolon)

                '0'..='9' => {
                    let start = self.index;
                    let mut raw_number = String::from(next_char);
                    while self.peek().is_numeric() {
                        raw_number.push(self.next().unwrap()); 
                    }
                    let number = raw_number.parse().unwrap();
                    self.push_long(Token::Literal(number), start);
                }

                'a'..='z' | 'A'..='Z' => {
                    let identifier = String::new();
                }
                _ => {} 
            };
        }             

        self.tokens.clone()
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(Positioned { inner: token, range: self.index..self.index })
    } 

    pub fn push_long(&mut self, token: Token, start: usize) {
        self.tokens.push(Positioned { inner: token, range: start..self.index })
    } 

    pub fn eof_error(&self) -> Error {
        Error {
            message: String::from("Expected token found EOF"),
            range: (self.index - 1)..(self.index)
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        match self.program.len() > self.index {
            true => Some(self.program[self.index - 1]),
            false => None,
        } 
    }

    pub fn peek(&mut self) -> Option<char> {
        match self.program.len() > self.index {
            true => Some(self.program[self.index]),
            false => None
        } 
    }
}

trait IsNumeric {
    fn is_numeric(&self) -> bool;
}

impl IsNumeric for Option<char> {
    fn is_numeric(&self) -> bool {
        match self {
            Some(c) => c.is_numeric(),
            None => false,
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Literal(u8),
    Identifier(String),
    
    OpenParen,
    CloseParen,
    Semicolon,

    Percent,
    Carat,

    And,
    Or,

    Plus,
    Add,
    Asterisk,
    Slash,
}
