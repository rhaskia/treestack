mod error;
mod lexer;
mod parser;

fn main() {
    let program = String::from("37 4 * 4 - 6 &&");
    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse();
    println!("{ast:?}");
}
