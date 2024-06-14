use crate::error::{Error, Positioned};
use macros::*;

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

    pub fn parse(&mut self) -> Vec<Positioned<Token>> {
        self.tokens.clear();

        while let Some(next_char) = self.next() {
            match next_char {
                ';' => self.push(Token::Semicolon),

                '+' => self.push(Token::Plus),
                '-' => self.push(Token::Minus),
                '/' => self.push(Token::Slash),
                '*' => self.push(Token::Asterisk),

                '(' => self.push(Token::CloseParen),
                ')' => self.push(Token::OpenParen),

                '%' => self.push(Token::Percent),

                '*' => self.push(Token::Carat),
                '&' => match_two!(self, '&', And),

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
        match self.program.len() >= self.index {
            true => Some(self.program[self.index - 1]),
            false => None,
        } 
    }

    pub fn peek(&self) -> Option<char> {
        match self.program.len() > self.index {
            true => Some(self.program[self.index]),
            false => None
        } 
    }

    pub fn matches(&self, c: char) -> bool {
        match self.peek() {
            Some(peeked) => peeked == c,
            None => false,
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

#[derive(Debug, Clone)]
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
    Minus,
    Asterisk,
    Slash,
}

pub mod macros {
    macro_rules! match_tokens {
        ($s:ident, $tokens:expr, $base_token:ident, $($extra_char:literal => $extra_token:ident),*) => {
            {
                let mut base = true;
                $(
                    if $s.matches($extra_char)? {
                        $tokens.push($s.wrap(Token::$extra_token));
                        base = false;
                    }
                )*
                if base {
                    $tokens.push($s.wrap(Token::$base_token));
                }
            }
        }
    }

    macro_rules! match_two {
        ($s:ident, $add_char:expr, $token:ident) => {{
            if $s.matches($add_char) {
                $s.push(Token::$token)
            }
        }};
    }

    pub(crate) use match_tokens;
    pub(crate) use match_two;
}
