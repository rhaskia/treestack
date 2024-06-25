mod error;
mod lexer;
mod tree;
mod interpreter;
mod parser;
//mod compiler;
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
}

fn run_file(file: &str, debug: bool) {
    let program = match load_file(file) {
        Ok(file) => file,
        Err(err) => { eprintln!("{}", err); return; }
    };

    let tokens = Lexer::new(program).parse();
    if debug { println!("{tokens:?}"); }// FIT behind debug flag

    let ast = match parser::Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(err) => { eprintln!("{:?}", err.pretty()); return; }
    };
    if debug { println!("{ast:?}"); } // FIT behind debug flag

    let result = Interpreter::new(debug).parse(ast);
    if let Err(msg) = result {
        eprintln!("{msg:?}");
    }
}

fn load_file(path: &str) -> Result<String, String> {
   std::fs::read_to_string(path).map_err(|e| format!("Error while loading program: {:?}", e.kind()))
}
