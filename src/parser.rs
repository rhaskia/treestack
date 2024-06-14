use crate::error::{Error, Positioned};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Positioned<Token>>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Positioned<Token>>) -> Self {
        Self { tokens, index: 0 }
    }    

    pub fn parse(&mut self) -> Vec<Positioned<Node>> {
        let mut nodes = Vec::new();

        while let Some(token) = self.next() {
            match token {
                _ => nodes.push(self.expression()),
            }         
        }

        nodes
    }

    pub fn expression(&mut self) -> Positioned<Node> {
        let start = self.previous();

        Positioned {
            inner: Node::Operator,
            range: 0..01
        }
    } 

    pub fn previous(&mut self) -> Option<Positioned<Token>> {
        match self.tokens.len() >= self.index {
            true => Some(self.tokens[self.index - 1].clone()),
            false => None,
        } 
    }

    pub fn next(&mut self) -> Option<Positioned<Token>> {
        self.index += 1;
        self.previous()
    }

    pub fn peek(&self) -> Option<&Positioned<Token>> {
        match self.tokens.len() > self.index {
            true => Some(&self.tokens[self.index]),
            false => None
        } 
    }
}

#[derive(Debug)]
pub enum Node {
    Push(u8),
    Operator,
    Function,
}
