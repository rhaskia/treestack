mod error;
mod lexer;
mod tree;
mod intrepreter;
mod parser;
mod compiler;

fn main() {
    let program = String::from("240 3 + v 48 ^ 2 + .");
    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse().unwrap();
    println!("{ast:?}");
    intrepreter::Interpreter::new().parse(&ast);
    // let llvm_ir = compiler::Compiler::new().compile(&ast);
    // std::fs::write("./output.ll", llvm_ir).unwrap();
}
