mod error;
mod lexer;
mod tree;
mod intrepreter;
mod parser;
mod compiler;

fn main() {
    let file = get_args();
    let program = load_file(&file[1]);

    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse().unwrap();
    println!("{ast:?}");
    intrepreter::Interpreter::new().parse(&ast);

    // let llvm_ir = compiler::Compiler::new().compile(&ast);
    // std::fs::write("./output.ll", llvm_ir).unwrap();
}

fn get_args() -> Vec<String> {
   std::env::args().collect::<Vec<String>>()
}

fn load_file(path: &str) -> String {
   std::fs::read_to_string(path).expect("File not found")
}
