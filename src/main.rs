mod error;
mod lexer;
mod tree;
mod interpreter;
mod parser;
mod compiler;
mod repl;
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;

fn main() {
    let args = get_args();

    // Proper Clap stuff
    if args.len() > 1 {
       run_file(&args[1]) 
    } else {
        repl::start_repl();
    }

    // let llvm_ir = compiler::Compiler::new().compile(&ast);
    // std::fs::write("./output.ll", llvm_ir).unwrap();
}

fn run_file(file: &str) {
    let program = load_file(file);

    let tokens = Lexer::new(program).parse();
    println!("{tokens:?}"); // FIT behind debug flag

    let ast = Parser::new(tokens).parse().unwrap();
    println!("{ast:?}"); // FIT behind debug flag

    Interpreter::new().parse(ast);
}

fn get_args() -> Vec<String> {
   std::env::args().collect::<Vec<String>>()
}

fn load_file(path: &str) -> String {
   std::fs::read_to_string(path).expect("File not found")
}
