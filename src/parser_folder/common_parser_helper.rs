use crate::lexer_folder::lexer::Token;
use super::parser::Parser;

impl Parser {
    pub fn expect(&mut self, expected: Token) -> Result<(), String> {
        let tok = self.advance();

        if tok == expected {
            return Ok(());
        } else {
            Err(format!("Expected {:?} but got {:?}", expected, tok))
        }
    }
}
