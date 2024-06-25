mod error;
mod lexer;
mod tree;
mod interpreter;
mod parser;
mod compiler;
mod repl;
//use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;
use clap::{Parser, command, arg};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Proper Clap stuff
    if let Some(ref file) = args.file {
       run_file(&file, args.debug); 
    } else {
        repl::start_repl(args.debug);
    }

    // let llvm_ir = compiler::Compiler::new().compile(&ast);
    // std::fs::write("./output.ll", llvm_ir).unwrap();
}

fn run_file(file: &str, debug: bool) {
    let program = load_file(file);

    let tokens = Lexer::new(program).parse();
    if debug { println!("{tokens:?}"); }// FIT behind debug flag

    let ast = parser::Parser::new(tokens).parse().unwrap();
    if debug { println!("{ast:?}"); } // FIT behind debug flag

    Interpreter::new(debug).parse(ast);
}

fn get_args() -> Vec<String> {
   std::env::args().collect::<Vec<String>>()
}

fn load_file(path: &str) -> String {
   std::fs::read_to_string(path).expect("File not found")
}
