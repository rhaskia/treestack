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
            match char {
                '0'..='9' => {
                    let digit = chars[self.index] as u8 - ('0' as u8);
                    while self.peek().is_numeric() {

                    }
                    tokens.push(Token::Literal(digit));
                }
                _ => {}
            }
        }             

        tokens
    }

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        match self.program.len() > self.index {
            true => Some(self.program[self.index - 1]),
            false => None
        } 
    }
}

#[derive(Debug)]
pub enum Token {
    Literal(u8),
    Operator,
}
