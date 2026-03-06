// So, now here is the REPL(Read-Eval-Print-Loop) of the CosmicLang
// This is the part user interact with.

// Module imports
mod evaluator;
mod lexer;
mod parser;

// things we would be using from each module
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write}; // to read inputs anf flushing stdout

fn main() {
    // Some cool welcome banner 😎
    intro();

    // creating a common evaluator so that variables stay aline between lines
    let mut evaluator = Evaluator::new();

    // The endless loop of REPL
    loop {
        // So, here flush() is used to print immediately because RUST buffers the output.
        print!("cosmic >> ");
        io::stdout().flush().unwrap();

        // Read the given line
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove leading and traling white spaces
        let input = input.trim();

        // handling commands
        match input {
            "" => continue,

            "exit" | "quit" => {
                println!("Have a nice day 👋");
                break;
            }

            "vars" => {
                // show all stored variables
                let env = evaluator.get_env();
                if env.is_empty() {
                    println!("No stored variables");
                } else {
                    println!("Stored variables are : ");
                    for (name, val) in env {
                        println!("{} => {}", name, val);
                    }
                }
                continue;
            }

            "clear" => {
                // clearing screen with ANSI(American National Standards Institute) escape code
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
                continue;
            }

            _ => {} // fall to the compiler pipeline 
        }

        // Compiler pipeline

        // Step 1: Lexar
        let mut lexar = Lexer::new(input);
        let tokens = lexar.tokenize();

        // Step 2: Parser
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(expr) => expr,
            Err(e) => {
                println!("Parse error: {}", e);
                continue; // to the next REPL iteration
            }
        };

        // Step 3: Evaluate
        match evaluator.eval(ast) {
            Ok(res) => {
                // println!("{}", res)
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn intro() {
    println!("╔══════════════════════════════════════╗");
    println!("║           CosmicLang  v1.0.0         ║");
    println!("╠══════════════════════════════════════╣");
    println!("║  Try: 2 + 3 * 4                      ║");
    println!("║       x = 10                         ║");
    println!("║       x + 5                          ║");
    println!("║       (x + 2) * 3                    ║");
    println!("║  Commands: 'vars' | 'clear' | 'exit' ║");
    println!("╚══════════════════════════════════════╝");
    println!();
}