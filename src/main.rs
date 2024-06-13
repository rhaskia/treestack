mod error;
mod lexer;
mod parser;

fn main() {
    let program = String::from("3 4 *");
    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
}
