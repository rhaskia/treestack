mod error;
mod lexer;
mod tree;
mod intrepreter;
mod parser;

fn main() {
    let program = String::from("240 while { 1 SWAP - DUP . }");
    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse().unwrap();
    println!("{ast:?}");
    intrepreter::Interpreter::new().parse(&ast);
}
