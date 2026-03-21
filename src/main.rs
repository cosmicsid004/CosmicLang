// So, now here is the REPL(Read-Eval-Print-Loop) of the CosmicLang
// This is the part user interact with.

// Module imports
mod evaluator_folder;
mod lexer_folder;
mod parser_folder;
mod stmt;
mod value;

// things we would be using from each module
use evaluator_folder::evaluator::Evaluator;
use lexer_folder::lexer::Lexer;
use parser_folder::parser::Parser;
use std::io::{self, Write}; // to read inputs anf flushing stdout

use std::env; // use this to read command line input
use std::fs; // use this to work with files (reading, writing etc)

use crate::parser_folder::parser::{self, Expr}; 


fn main () {

    // this stors the arguments passed in command line
    let args: Vec<String> = env::args().collect();

    // args[0] is always the binary name, so we directly jumo to index 1
    match args.get(1).map(|s| s.as_str()) { //converts Option<&String> into Option<&str>
        Some("repl") => {
            start_repl();
        }
        Some("compile") => {
            match args.get(2) {
                Some(filename) => {
                    compile_file(filename);
                }
                None => {
                    eprintln!("Usage: cosmic compile");
                    std::process::exit(1); // terminates the code '0' means sucess and non-zero is error
                }
            }
        }
        _ => {
            print_help();
        }
    }

}

fn start_repl() {
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
        run_pipeline(input, &mut evaluator, true)
    }
}

fn compile_file(filename: &str) {

    // enforce to have .cosmic extension
    if !filename.ends_with(".cosmic") {
        eprintln!("Error: CosmicLang files must have a .cosmic extension");
        std::process::exit(1);
    }

    // read the file
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Could not read '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    println!("Compiling: {}", filename);
    println!("----------------------------------------");

    // runnig each line through pipeline
    let mut evaluator = Evaluator::new();
    for (line_num, line) in source.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Err(e) = run_pipeline_result(line, &mut evaluator) {
            eprint!("[Line {}] Error: {}", line_num + 1, e);
            std::process::exit(1); // stop immeditly after encountring an error
        } 
    }

    println!("-----------------------------------------");
    println!("Execution completed");
}

// pipline used by REPL
fn run_pipeline(source: &str, evaluator: &mut Evaluator, print_result: bool) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(expr) => expr,
        Err(e) => {
            println!("Parse error: {}", e);
            return;
        }
    };

    match evaluator.eval_stmt(ast) {
        Ok(res) => {
            if print_result {
                // REPL print
                // println!("{}", res);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// pipeline used by compiler
fn run_pipeline_result(source: &str, evaluator: &mut Evaluator) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    evaluator.eval_stmt(ast).map_err(|e| e.to_string());

    Ok(())
}

fn intro() {
    println!("╔══════════════════════════════════════╗");
    println!("║       CosmicLang  v1.0.0             ║");
    println!("╚══════════════════════════════════════╝");
    println!();
}

fn print_help() {
    println!("╔══════════════════════════════════════╗");
    println!("║       CosmicLang  v1.0.0             ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("Usage:");
    println!("  cosmic repl                -> Start interactive REPL");
    println!("  cosmic compile <file.cz>   -> Compile and run a .cosmic file");
    println!();
}