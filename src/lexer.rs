use fehler::throws;

pub struct Lexer {
    index: usize,
    program: Vec<char>,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        let program = program.chars().collect();
        Lexer { index: 0, program }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(char) = self.next() {
        }             

        tokens
    }

    pub fn match_token() -> Option<Token> {
        match char {
            '0'..='9' => {
                let digit = chars[self.index] as u8 - ('0' as u8);
                while self.peek()?.is_numeric() {

                }
                tokens.push(Token::Literal(digit));
            }
            _ => {}
        }

        None
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

#[derive(Debug)]
pub enum Token {
    Literal(u8),
    Operator,
}
