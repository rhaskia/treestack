mod error;
mod lexer;
mod parser;

fn main() {
    let program = String::from("3 74 * 6 > if { 3 4 . } else { 3 4 . } 5 6 + while { . 4 }");
    let tokens = lexer::Lexer::new(program).parse();
    println!("{tokens:?}");
    let ast = parser::Parser::new(tokens).parse();
    println!("{ast:?}");
}
