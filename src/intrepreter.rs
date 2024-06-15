use crate::error::Positioned;
use crate::lexer::Token;
use crate::parser::Node;
use crate::tree::TreeNode;

pub struct Interpreter {
    stack: TreeNode<u8>,
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { stack: TreeNode::new(), pointer: 0 }
    }

    pub fn parse(&mut self, instructions: &Vec<Positioned<Node>>) {
        for instruction in instructions {
            match &instruction.inner {
                Node::Expression(expr) => self.parse(expr),
                Node::Push(u) => self.stack.push_raw(*u),
                Node::Operator(op) => self.eval_op(op.clone()),
                Node::Call(call) => self.call(call),
                Node::While(expr) => while self.truthy() { self.parse(expr) },
                Node::If(if_expr, else_expr) => {
                    if self.truthy() {
                        self.parse(if_expr)
                    } else {
                        if let Some(expr) = else_expr {
                            self.parse(expr);
                        }
                    }
                }
                Node::Function => todo!(),
            }
        }
    }

    pub fn call(&mut self, call: &str) {
        match call {
            "SWAP" => {
                let first = self.pop();
                let second = self.pop();
                self.stack.push(first);
                self.stack.push(second);
            }
            "DUP" => {
                let first = self.stack.last().unwrap().clone();
                self.stack.push(first);
            }
            _ => {
                // TODO check functions
            }
        }
    }

    pub fn truthy(&self) -> bool {
        self.stack.last().map(|v| v.val > 0).unwrap_or(false)
    }

    pub fn eval_op(&mut self, op: Token) {
        match op {
            Token::OpenParen => todo!(),
            Token::CloseParen => todo!(),
            Token::OpenBrace => todo!(),
            Token::CloseBrace => todo!(),
            Token::Semicolon => todo!(),
            Token::Percent => todo!(),
            Token::And => todo!(),
            Token::Or => todo!(),
            Token::Plus => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.stack.push(lhs + rhs);
            },
            Token::Minus => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.stack.push(lhs - rhs);
            },
            Token::Asterisk => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.stack.push(lhs * rhs);
            }
            Token::Slash => todo!(),
            Token::Period => print!("{}", self.pop()),
            _ => {}
        }
    }

    pub fn pop(&mut self) -> TreeNode<u8> {
        self.stack.pop().unwrap()
    }
}
