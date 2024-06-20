use std::ops::Range;

use crate::error::{Error, Positioned, position};
use crate::lexer::{Keyword, Token};
use fehler::throws;

pub struct Parser {
    tokens: Vec<Positioned<Token>>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Positioned<Token>>) -> Self {
        Self { tokens, index: 0 }
    }

    #[throws]
    pub fn parse(&mut self) -> Vec<Positioned<Node>> {
        self.expression()?
    }

    #[throws]
    pub fn expression(&mut self) -> Vec<Positioned<Node>> {
        //let start = self.previous();
        let mut expr = Vec::new();

        while let Some(token) = self.next() {
            let Positioned { range, inner } = token;
            match inner {
                Token::Literal(l) => expr.push(position(Node::Push(l), range)),
                Token::Word(w) => expr.push(position(Node::Call(w), range)),
                Token::Keyword(k) => expr.push(self.statement(k)?),
                Token::CloseBrace => break,
                op => expr.push(position(Node::Operator(op), range)),
            }
        }

        expr
    }

    #[throws]
    pub fn statement(&mut self, keyword: Keyword) -> Positioned<Node> {
        let start = self.previous().unwrap().range.start;
        self.ensure_next(Token::OpenBrace)?;

        let node = match keyword {
            Keyword::If => {
                let if_expr = self.expression()?;
                let mut else_expr = None;

                if self.peek().map(|t| **t == Token::Keyword(Keyword::Else)).unwrap_or(false) {
                    self.next();
                    self.ensure_next(Token::OpenBrace)?;
                    else_expr = Some(self.expression()?);
                }

                Node::If(if_expr, else_expr)
            },
            Keyword::Else => unreachable!(),
            Keyword::While => Node::While(self.expression()?),
        };
        let end = self.previous().unwrap().range.end;

        Positioned { inner: node, range: start..end }
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

    #[throws]
    pub fn ensure_next(&mut self, token: Token) -> Positioned<Token> {
        let next = self.next().unwrap();
        if next.inner == token { 
            return next;
        }
        return Err(self.error(format!("Expected {token:?} but found {:?}", next.inner), next.range))?
    }

    pub fn error(&self, message: String, range: Range<usize>) -> Error {
        Error { message, range }
    }

    pub fn peek(&self) -> Option<&Positioned<Token>> {
        match self.tokens.len() > self.index {
            true => Some(&self.tokens[self.index]),
            false => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Vec<Positioned<Node>>),
    Push(i64),
    Operator(Token),
    Call(String),
    While(Vec<Positioned<Node>>),
    If(Vec<Positioned<Node>>, Option<Vec<Positioned<Node>>>),
    Function(String, Vec<Positioned<Node>>),
}
