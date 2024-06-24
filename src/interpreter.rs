use crate::error::Positioned;
use crate::lexer::{PointerAction, Token};
use crate::parser::Node;
use crate::tree::TreeNode;
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::ops;
use syscalls::{raw_syscall, Sysno};
use fehler::throws;

type Error = String;

#[derive(Default, Clone, Debug)]
struct Pointer {
    pub tree: Vec<usize>,
    pub branch: usize,
}

impl Pointer {
    pub fn open_branch(&mut self, len: usize) {
        self.tree.push(self.branch);
        self.branch = len;
    }

    pub fn close_branch(&mut self) {
        self.branch = self.tree.pop().unwrap(); // Error
    }
}

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

    #[throws]
    pub fn parse(&mut self, instructions: Vec<Positioned<Node>>) {
        for instruction in instructions.into_iter() {
            let inst = format!("{:?}: ", instruction);
            match instruction.inner {
                Node::Expression(expr) => self.parse(expr)?,
                Node::Push(u) => self.push_raw(u),
                Node::String(string) => self.push_string(string),
                Node::Operator(op) => self.eval_op(op.clone())?,
                Node::Call(call) => self.call(&call)?,
                Node::While(expr) => {
                    while self.truthy() {
                        self.parse(expr.clone())?
                    }
                }
                Node::If(if_expr, else_expr) => {
                    if self.truthy() {
                        self.parse(if_expr.clone())?
                    } else {
                        if let Some(expr) = else_expr {
                            self.parse(expr.clone())?;
                        }
                    }
                }
                Node::Function(name, f) => {
                    self.functions.insert(name, f);
                }
                Node::Pointer(name, action) => self.call_pointer(name, action),
            }
            //println!("{inst}: {}, {:?}", self.stack, self.pointer);
        }
    }

    #[throws]
    pub fn call(&mut self, call: &str) {
        match call {
            "swap" => {
                let first = self.pop()?;
                let second = self.pop()?;
                self.push(first);
                self.push(second);
            }
            "dup" => {
                let first = self.stack.last().unwrap().clone();
                self.push(first);
            }
            "open" => {
            }
            "write" => {
            }
            "syscall" => {
                let call = self.pop()?.val;
                unsafe { 
                    let result = raw_syscall!(Sysno::from(call as i32)); 
                    self.push_raw(result as i64); 
                }
                
            }
            "print" =>{
                print!("{}", self.pop_string()?);
            }
            _ => {
                let function = match self.functions.get(call) {
                    Some(f) => f,
                    None => return,
                };

                self.parse(function.clone())?;
            }
        }
    }

    #[throws]
    pub fn pop_string(&mut self) -> String {
        let length = self.pop()?.val;
        let mut string = String::new();

        for _ in 0..length {
            let c = char::from_u32(self.pop()?.val as u32).unwrap();
            string.insert(0, c);
        }

        string
    }

    pub fn call_pointer(&mut self, name: String, action: PointerAction) {
        match action {
            PointerAction::Jump => {
                self.pointer = self.pointers[&name].clone();
            }
            PointerAction::Create => {
                self.pointers.insert(name, self.pointer.clone());
            }
            PointerAction::Push => {
                let pointer = self.pointers[&name].clone();
                let value = self.at_pointer(pointer).clone();
                self.push(value)
            }
        }
    }

    pub fn current(&mut self) -> &mut TreeNode<i64> {
        self.at_pointer(self.pointer.clone())
    }

    pub fn at_pointer(&mut self, pointer: Pointer) -> &mut TreeNode<i64> {
        let mut head = &mut self.stack;
        for pointer in &pointer.tree {
            head = &mut head.children[*pointer];
        }
        head
    }

    pub fn truthy(&mut self) -> bool {
        let branch = self.pointer.branch;
        self.current()[branch].val > 0
    }

    pub fn push_raw(&mut self, val: i64) {
        self.push(TreeNode { val, children: Vec::new() });
    }

    pub fn push_string(&mut self, string: String) {
        let length = string.len();
        for char in string.chars() {
            self.push_raw(char as i64);
        }
        println!("{length}");
        self.push_raw(length as i64);
    }

    pub fn push(&mut self, node: TreeNode<i64>) {
        self.current().push(node);
    }

    #[throws]
    pub fn eval_op(&mut self, op: Token) {
        use Token::*;
        match &op {
            Period => print!("{}", self.pop()?),
            Comma => {} // Read Char (not top priority rn)
            OpenBracket => {
                let len = self.current().len();
                self.pointer.open_branch(len);
            }
            CloseBracket => self.pointer.close_branch(),
            OpenParen => self.pointer.branch += 1,
            CloseParen => self.pointer.branch -= 1,
            Semicolon => todo!(),
            Plus | Asterisk | Minus | Slash | Or | And | Percent => {
                let lhs = self.pop()?;
                let rhs = self.pop()?;
                let func = op.func();
                self.push(lhs.eval(rhs, func));
            }
            _ => {
                println!("Unused Operator: {op:?}");
            }
        }
    }

    pub fn pop(&mut self) -> Result<TreeNode<i64>, Error> {
        match self.current().pop() {
            Some(tn) => Ok(tn),
            None => self.error("Stack underflow"),
        }
    }

    pub fn error<T>(&self, msg: &str) -> Result<T, Error> {
        Err(format!("Encountered runtime error: {msg}"))
    }
}

impl Token {
    pub fn func(&self) -> fn(i64, i64) -> i64 {
        use Token::*;
        match self {
            Plus => ops::Add::add,
            Asterisk => ops::Mul::mul,
            Minus => ops::Sub::sub,
            Slash => ops::Div::div,
            Percent => ops::Rem::rem,
            And => |l, r| (l > 0 && r > 0) as i64,
            Or => |l, r| (l > 0 || r > 0) as i64,
            _ => unreachable!(),
        }
    }
}
