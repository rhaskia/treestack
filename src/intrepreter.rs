use crate::error::Positioned;
use crate::lexer::Token;
use crate::parser::Node;
use crate::tree::TreeNode;

pub struct Interpreter {
    stack: TreeNode<i64>,
    pointers: Vec<usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { stack: TreeNode::default(), pointers: Vec::new() }
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
                Node::Function(f) => todo!(),
            }
            println!("{:?}: {}, {:?}", instruction, self.stack, self.pointers);
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
                // TODO check functions
            }
        }
    }

    pub fn current(&mut self) -> &mut TreeNode<i64> {
        let mut head = &mut self.stack;
        for pointer in &self.pointers {
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
            Token::CloseBracket => { self.pointers.pop(); },
            Token::OpenBracket => {
                let last = self.current().children.len() - 1;
                self.pointers.push(last);
            }
            _ => {}
        }
    }

    pub fn pop(&mut self) -> TreeNode<i64> {
        self.current().pop().unwrap()
    }
}
