//Step 2

// So, now lets move on to the Parser CosmicLang
// This converts the list of Tokens to an AST(Abstract Syntax Tree)
// This is a classic Recursive Decent Parser
// We do this to capture the MEANING and PRECEDENCE of the expression.
// For instance 2 + 4 * 3 should NOT be (2 + 4) * 3 = 18
//                        insted it should be 2 + (4 * 3) = 14
// So, we will make a tree and calculate it bottom up
use crate::lexer_folder::lexer::Token; // this means we are refreing a file in root file
use crate::stmt::Stmt;

// So, the tree can have any the following types of node
// this is recursive enum, that's why this is using a Box<Expr>
// Expr can contain another Expr in it which makes it recursive in nature.
// Box<T> puts the data in the heap and pointer in the Stack, so the RUST doesn'r complain.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    StringLiteral(String),
    Variable(String),
    Assign(String, Box<Expr>),
    BinOp(Box<Expr>, Op, Box<Expr>),
    // Publish(Box<Expr>)
}

// Math operstors
#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    NotEqualTo,
    EqualEqual
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    // constructor for the Parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // looking at current token without consuming.
    // just looking to decide what ro do next
    fn current(&self) -> &Token {
        &self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    // we are returning the current token and moving forward/consuming
    pub fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    // Grammer
    // programe     = expression | assignment               {a programe can be an (5 + 2) or (x = 8)}
    // assignment   = Ident "=" expression                  {it can be x = (6 + 2) * 8}
    // comparision  = expression(('<' | '>') expression)*   {it can be x > 10}
    // expression   = term (('+' | '-') term)*              {it can be (10 + 5) the star at end idicate it can be multiple times}
    // term         = unary (('*' | '/') unary)*            {it can be (8 * 9)}
    // unary        = '-' unary | primary                   {it can be -5, -(-8)}
    // primary      = Number | Ident | '(' expression ')'   {the most basic thing 6, (8 + 2)}

    // Lower in the list = higher precedence.

    // Start parsing
    pub fn parse(&mut self) -> Result<Stmt, String> {
        // check for publish keyword
        if let Token::Publish = self.current() {
            self.advance();
            self.expect(Token::LParen)?;
            let expr = self.parse_comparision()?;
            self.expect(Token::RParen)?;
            return Ok(Stmt::Publish(expr)); // return Stmt not Expr
        }

        // everything else is a statement wrapping an expression
        let expr = self.parse_assignment()?;
        Ok(Stmt::Expression(expr))
    }

    // handling assignment i:e x = expr
    fn parse_assignment(&mut self) -> Result<Expr, String> {
        // checking it it's in the form "ident ="
        if let Token::Ident(name) = self.current().clone() {
            if self.pos + 1 < self.tokens.len() {
                // check if there is next token available
                if self.tokens[self.pos + 1] == Token::Equal {
                    // check if there is '=' sign
                    self.advance(); // consumes Ident
                    self.advance(); // consumes '='
                    let val = self.parse_expr()?;
                    return Ok(Expr::Assign(name, Box::new(val)));
                }
            }
        }
        // not ans assignment
        self.parse_comparision()
    }

    // handling comparision operations
    fn parse_comparision(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_expr()?;

        loop {
            match self.current() {
                Token::RAnchor => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::GreaterThan, Box::new(right));
                }
                Token::LAnchor => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::LessThan, Box::new(right));
                }
                Token::EqualEqual => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::EqualEqual, Box::new(right));
                }
                Token::NotEqualTo => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::NotEqualTo, Box::new(right))
                }
                Token::RAnchorEqual => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::GreaterThanEqual, Box::new(right));
                }
                Token::LAnchorEqual => {
                    self.advance();
                    let right = self.parse_expr()?;
                    left = Expr::BinOp(Box::new(left), Op::LessThanEqual, Box::new(right));
                }
                _ => break
            }
        }
        Ok(left)
    }

    // handling: term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<Expr, String> {
        // reading the first value, the expression must start with a term {(5 + 3) so, left = 5}
        let mut left = self.parse_term()?;

        loop {
            match self.current() {
                // if it's a '+' we parse and calculate the right side
                Token::Plus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::BinOp(Box::new(left), Op::Add, Box::new(right));
                }
                // if it's a '-' we parse and calculate the right side
                Token::Minus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::BinOp(Box::new(left), Op::Sub, Box::new(right));
                }
                // no '+' or '-' then break
                _ => break,
            }
        }

        // final expression tree
        Ok(left)
    }

    // handling: term unary (('*' | '/') unary)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            match self.current() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp(Box::new(left), Op::Mul, Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp(Box::new(left), Op::Div, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // handling: '-' unary | primary
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if let Token::Minus = self.current() {
            self.advance();
            let operand = self.parse_unary()?; // handlling cases like --5
            // Turning numbers like -5 to (0 - 5)
            return Ok(Expr::BinOp(
                Box::new(Expr::Number(0.0)),
                Op::Sub,
                Box::new(operand),
            ));
        }
        self.parse_primary()
    }

    //handling: Number | Ident | '(' expression ')'
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)), // we are matching Token::Number(n) with self.advance() and if the Token matches we return
            Token::StringLiteral(s) => Ok(Expr::StringLiteral(s)),
            Token::Ident(name) => Ok(Expr::Variable(name)),

            Token::LParen => {
                let expr = self.parse_expr()?; // parsing whats inside the '()'
                match self.advance() {
                    Token::RParen => Ok(expr), //consume the closing ')'
                    other => Err(format!("Expected ')' but got {:?}", other)), //finding something unrelated
                }
            }
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }
}
