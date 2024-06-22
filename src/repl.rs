use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;
use std::io::{stdin, stdout, Write};

pub fn start_repl() {
    let mut interpreter = Interpreter::new();
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut line = String::new();
    
    loop {
        print!("\n> ");
        stdout.flush();
        stdin.read_line(&mut line);

        let tokens = Lexer::new(line.clone()).parse();
        let ast = Parser::new(tokens).parse().unwrap();
        interpreter.parse(ast);

        line.clear();
    } 
}
