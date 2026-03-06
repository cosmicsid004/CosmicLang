// Step 3

// So, the evaluator walks through the AST and computes the final value
// It als holds a HashMapof variables names -> values

use std::collections::HashMap;
use crate::parser::{Expr, Op};

pub struct Evaluator {
    // the Hashmap maps the variable names to their values
    // for now we have only one scope that is GLOBAL
    env: HashMap<String, f64>
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: HashMap::new() // Assigning new HashMap
        }
    }

    // So, the main method will evaluate the node of Expr AST recursively
    // Returns: Result<f64, String>
    //              Ok(number): sucess
    //              Err(message): something went wrong
    pub fn eval(&mut self, expr: Expr) -> Result<f64, String> {
        match expr {
            // Only number - just returning it
            Expr::Number(n) => Ok(n),

            // A variable - check if present in the environment
            Expr::Variable(name) => {
                match self.env.get(&name) {
                    Some(val) => Ok(*val), // found it return the value, here derefrending is done because we promised to return a f64 not a &f64
                    None => Err(format!("Undefined variable {}", name))
                }
            }

            // An assignment: x = expr
            // So, first evaluate the right then store it then return it
            Expr::Assign(name, value_expr) => {
                let val = self.eval(*value_expr)?; // recurursively solving the nested statements. Derefrencing because eval takes Expr not Box<Expr>
                self.env.insert(name.clone(), val);
                println!("{} = {}", name, val); // display what was stored
                Ok(val)
            }

            // A binary operation: left Op right
            Expr::BinOp(left, op, right) => {
                let l = self.eval(*left)?; // evaluate the left subtree
                let r = self.eval(*right)?; // evaluate the right subtree 

                match op {
                    Op::Add => Ok(l + r),
                    Op::Sub => Ok(l - r),
                    Op::Mul => Ok(l * r),
                    Op::Div => {
                        if r == 0.0 {
                            Err("Zero division error".to_string())
                        } else {
                            Ok(l / r)
                        }
                    }
                }
            }
        }
    }

    // A getter method to see what variables are stored
    pub fn get_env(&self) -> &HashMap<String, f64> {
        &self.env
    }
}