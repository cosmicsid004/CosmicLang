mod lexar;
mod parser;

use lexar::{Lexer, Token};
use parser::{Parser};
fn main() {
    let source_code = "x = (2 + 2) * 2";
    let mut lexar = Lexer::new(source_code);
    let tokens = lexar.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("AST:");
            println!("{:#?}", ast);
        }
        Err(e) => println!("Parse error {}", e)
    }
}
