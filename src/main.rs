mod lexar;
mod parser;
mod evaluator;

use lexar::{Lexer, Token};
use parser::{Parser};
use evaluator::Evaluator;
fn main() {
    let source_code = "x = 2 + 2 * 2";
    let mut lexar = Lexer::new(source_code);
    let tokens = lexar.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("AST:");
            println!("{:#?}", ast);

            //Evaluate AST
            let mut evaluator = Evaluator::new();

            match evaluator.eval(ast) {
                Ok(result) => {
                    println!("Result: {}", result);
                    println!("Environment: {:?}", evaluator.get_env());
                }
                Err(e) => println!("Runtime error: {}", e)
            }
        }
        Err(e) => println!("Parse error {}", e)
    }
}
