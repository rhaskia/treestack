use crate::parser::Node;
use crate::error::Positioned;

pub struct Compiler {
}

impl Compiler {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn compile(&mut self, program: &Vec<Positioned<Node>>) -> String {
        let mut ir = String::new();

        for instruction in program {
            match instruction {
                _ => {}
            }
        }

        ir
    }
}
