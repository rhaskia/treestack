use crate::error::Positioned;
use crate::lexer::{Token, PointerAction};
use crate::parser::Node;
use crate::tree::TreeNode;
use std::collections::HashMap;

type Pointer = Vec<usize>;

#[derive(Default)]
pub struct Interpreter {
    stack: TreeNode<i64>,
    functions: HashMap<String, Vec<Positioned<Node>>>,
    pointer: Pointer,
    pointers: HashMap<String, Pointer>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn parse(&mut self, instructions: &Vec<Positioned<Node>>) {
        for instruction in instructions {
            match &instruction.inner {
                Node::Expression(expr) => self.parse(expr),
                Node::Push(u) => self.push_raw(*u),
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
                Node::Function(name, f) => { self.functions.insert(name.clone(), f.clone()); }
                Node::Pointer(name, action) => self.call_pointer(name.clone(), *action),
            }
            println!("{:?}: {}, {:?}", instruction, self.stack, self.pointer);
        }
    }

    pub fn call(&mut self, call: &str) {
        match call {
            "SWAP" => {
                let first = self.pop();
                let second = self.pop();
                self.push(first);
                self.push(second);
            }
            "DUP" => {
                let first = self.stack.last().unwrap().clone();
                self.push(first);
            }
            "v" => {
            }
            _ => {
            }
        }
    }

    pub fn call_pointer(&mut self, name: String, action: PointerAction) {
        match action {
            PointerAction::Jump => {
                self.pointer = self.pointers[&name].clone();
            },
            PointerAction::Create => { self.pointers.insert(name, self.pointer.clone()); },
            PointerAction::Push => todo!(),
        }
    }

    pub fn current(&mut self) -> &mut TreeNode<i64> {
        let mut head = &mut self.stack;
        for pointer in &self.pointer {
            head = &mut head.children[*pointer];
        }
        head
    }

    pub fn truthy(&self) -> bool {
        self.stack.last().map(|v| v.val > 0).unwrap_or(false)
    }

    pub fn push_raw(&mut self, val: i64) {
        self.push(TreeNode { val, children: Vec::new() });
    }

    pub fn push(&mut self, node: TreeNode<i64>) {
        self.current().push(node);
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
                self.push(lhs + rhs);
            },
            Token::Minus => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.push(lhs - rhs);
            },
            Token::Asterisk => {
                let lhs = self.pop();
                let rhs = self.pop();
                self.push(lhs * rhs);
            }
            Token::Slash => todo!(),
            Token::Period => print!("{}", self.pop()),
            Token::CloseBracket => { self.pointer.pop(); },
            Token::OpenBracket => {
                let last = self.current().children.len() - 1;
                self.pointer.push(last);
            }
            _ => {}
        }
    }

    pub fn pop(&mut self) -> TreeNode<i64> {
        self.current().pop().unwrap()
    }
}
