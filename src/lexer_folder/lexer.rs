// Step 1

// So, this will used to make the convert the raw text into list of Tokens.
// For instance "x = 10 + 6" -> [Ident(x), Equal, Number(10), Plus, Number(6)]

//Token : enum will have all the Tokens possible in our lang.
// #[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),        // Any number: 3, 5

    LParen,             // (
    RParen,             // )

    Plus,               // +
    Minus,              // -
    Star,               // * 
    Slash,              // /
    Equal,              // =
    
    RAnchor,            // >
    LAnchor,            // <

    RAnchorEqual,       // >= 
    LAnchorEqual,       // <=
    EqualEqual,         // ==
    NotEqualTo,         // !=

    Ident(String),      // variable(identifier): x, y
    
    Publish,            // use this to get standard output

    EOF                 // End Of Input: to know when the line ends
}

// So, this will hold the text as well the current position of cursor
pub struct Lexer {
    input: Vec<char>, // source code split in idivisual tokens
    pos: usize, // Current pointer in the input Vector
}

impl Lexer {
    //so, here we will take a String slice and build a Lexer
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(), // -> 'a', 'b', 'c' -> ['a','b','c']
            pos: 0
        }
    }

    // so here we are check the charater at postion pos (wisthout taking its ownership)
    fn current(&self) -> Option<char> { // borowwing the Lexar
        self.input.get(self.pos).copied() 
        // we are copying because we promised to return Option<char> not Option<&char>
    }

    //so this function will read the nexr char and returns it wiout consuming it. We need this for operations with multiple char like ==, !=
    fn peek(&self) -> Option<char> {
        return self.input.get(self.pos + 1).copied()
    }

    // moving forward one char at a time
    fn advance(&mut self) {
        self.pos += 1;
    }

    // we have no use of space so we will just skip it
    fn skip_whitespace(&mut self) {
        // so we keep looping while self.current return a character
        while let Some(c) = self.current() {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Reading a number
    fn read_number(&mut self) -> Token {
        let mut num_string = String::new();

        while let Some(c) = self.current() {
            if c.is_ascii_digit() || c =='.' {
                num_string.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // so we are making it of Token type Number
        Token::Number(num_string.parse::<f64>().unwrap())
    }

    // Reading a full a variable/identifier
    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "publish" => Token::Publish,
            _ => Token::Ident(ident)  
        }
    }

    // This would be the MAIN METHOD which will tokenize the source code
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new(); // an empty Vectoer to store Tokens

        loop {
            self.skip_whitespace();

            match self.current() {
                // if returned None
                None => {
                    tokens.push(Token::EOF); // end of the String -> Completed
                    break;
                }

                Some(c) => match c {
                    '0'..='9' | '.' => tokens.push(self.read_number()),
                    'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.read_ident()),
                    '+' => { tokens.push(Token::Plus); self.advance() },
                    '-' => { tokens.push(Token::Minus); self.advance() },
                    '*' => { tokens.push(Token::Star); self.advance() },
                    '/' => { tokens.push(Token::Slash); self.advance() },
                    // '=' => { tokens.push(Token::Equal); self.advance() },
                    '(' => { tokens.push(Token::LParen); self.advance() }
                    ')' => { tokens.push(Token::RParen); self.advance() },
                    // '>' => { tokens.push(Token::RAnchor); self.advance(); }
                    // '<' => { tokens.push(Token::LAnchor); self.advance(); }
                    '>' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            self.advance();
                            tokens.push(Token::RAnchorEqual);
                        } else {
                            tokens.push(Token::RAnchor);
                        }
                    }
                    '<' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            self.advance();
                            tokens.push(Token::LAnchorEqual);
                        } else {
                            tokens.push(Token::LAnchor);
                            self.advance();
                        }
                    }
                    '=' => { 
                        if self.peek() == Some('=') {
                            self.advance();
                            self.advance();
                            tokens.push(Token::EqualEqual);
                        } else {
                            tokens.push(Token::Equal);
                            self.advance();
                        }
                     }
                     '!' => {
                        if self.peek() == Some('=') {
                            self.advance();
                            self.advance();
                            tokens.push(Token::NotEqualTo);
                        } else {
                            eprintln!("Unknown character: !");
                            self.advance();
                        }
                     }

                    other => {
                        eprintln!("Unknown character: {}", other);
                        self.advance() //skipping unknown character insted of crashing
                    }
                }
            }
        }

        tokens // return the completed list
    }
}

