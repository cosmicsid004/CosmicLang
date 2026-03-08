// Statement is something that says -> Do something
// Expression is something that says -> Compute Somehting

// Using htis we will be seperating both of these staements 

use crate::parser_folder::parser::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    // Just used to evaluate something {(x + 5)}
    Expression(Expr),

    // Not compute anything {(publish(8))}
    Publish(Expr),
}