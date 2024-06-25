use crate::error::Positioned;
use crate::lexer::{PointerAction, Token};
use crate::parser::Node;
use crate::tree::TreeNode;
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::ops;
use fehler::throws;
#[cfg(target_os = "linux")]
use syscalls::{raw_syscall, Sysno};

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
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Self { debug, ..Default::default() }
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
            if self.debug { println!("{inst}: {}, {:?}", self.stack, self.pointer); }
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
                let first = self.on()?.clone();
                self.push(first);
            }
            "read" => {
                let file = self.pop_string()?;
                self.push_file(std::fs::read(file).unwrap());
            }
            "write" => {
                let file = self.pop_string()?;
                let to_write = self.pop_string()?;
                std::fs::write(file, to_write);
            }
            "syscall" => {
                let call = self.pop()?.val;
                self.push_raw(syscall(call));
            }
            "abs" => {
                let val = self.on()?.val;
                self.on()?.val = val.abs();
            }
            "print" => {
                print!("{}", self.pop_string()?);
            }
            "group" => {
                let length = self.pop()?.val;
                let children: Result<Vec<TreeNode<i64>>, Error> = (0..length).map(|_| self.pop()).collect();
                self.push(TreeNode { val: 0, children: children? })
            }
            "rev" => {
                let rev_children = self.current().children.clone().into_iter().rev().collect();
                self.current().children = rev_children;
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
            head = &mut head.children[*pointer - 1];
        }
        head
    }

    pub fn truthy(&mut self) -> bool {
        let branch = self.pointer.branch;
        if branch == 0 { return false; }
        self.current()[branch - 1].val > 0
    }

    pub fn push_raw(&mut self, val: i64) {
        self.push(TreeNode { val, children: Vec::new() });
    }

    pub fn push_file(&mut self, vec: Vec<u8>) {
        let length = vec.len();
        for item in vec {
           self.push_raw(item as i64);
        }
        self.push_raw(length as i64);
    }

    pub fn push_string(&mut self, string: String) {
        let length = string.len();
        for char in string.chars() {
            self.push_raw(char as i64);
        }
        self.push_raw(length as i64);
    }

    pub fn push(&mut self, node: TreeNode<i64>) {
        let branch = self.pointer.branch;
        if branch <= self.current().len() { self.current().insert(branch, node); }
        else { self.current().push(node) }
        self.pointer.branch += 1;
    }

    #[throws]
    pub fn eval_op(&mut self, op: Token) {
        use Token::*;
        match &op {
            Period => print!("{}", self.pop()?),
            Comma => print!("{}", char::from_u32(self.pop()?.val as u32).unwrap()),
            OpenBracket => {
                let branch = self.pointer.branch;
                let len = self.current()[branch - 1].len();
                // BAD
                self.pointer.open_branch(len);
            }
            CloseBracket => self.pointer.close_branch(),
            OpenParen => self.pointer.branch += 1,
            CloseParen => self.pointer.branch -= 1,
            Semicolon => todo!(),
            PlusPlus => { self.on()?.val += 1; },
            MinusMinus => { self.on()?.val -= 1; },
            Not => { self.on()?.val = (self.on()?.val == 0) as i64 }
            Grave => { self.pop()?; }
            _ => {
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                let func = op.func();
                self.push(lhs.eval(rhs, func));
            }
        }
    }
    
    pub fn pop(&mut self) -> Result<TreeNode<i64>, Error> {
        let branch = self.pointer.branch;
        if self.current().children.is_empty() || branch == 0 { return self.error("Stack underflow"); }
        let value = self.current().remove(branch - 1);
        self.pointer.branch -= 1;
        Ok(value)
    }

    pub fn on(&mut self) -> Result<&mut TreeNode<i64>, Error> {
        let branch = self.pointer.branch;
        if self.current().children.is_empty() || branch == 0 { return self.error("Stack underflow"); }
        let value = &mut self.current()[branch - 1];
        Ok(value)
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
            Equals => |l, r| (l == r) as i64,
            Greater => |l, r| (l > r) as i64,
            Lesser => |l, r| (l < r) as i64,
            GreaterThan => |l, r| (l >= r) as i64,
            LesserThan => |l, r| (l <= r) as i64,
            _ => panic!("Operator not implemented {:?}", self),
        }
    }
}

#[cfg(target_os = "linux")]
fn syscall(call: i64) -> i64 {
                unsafe { 
                    let result = raw_syscall!(Sysno::from(call as i32)); 
                    result as i64
                }
}

#[cfg(not(target_os = "linux"))]
fn syscall(call: i64) -> i64 {
    -1
}