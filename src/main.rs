mod lexar;

use lexar::{Lexer, Token};
fn main() {
    let source_code = "x = 2 + 2";
    let mut lexar = Lexer::new(source_code);
    let tokens = lexar.tokenize();

    print!("{:?}", tokens);
}
