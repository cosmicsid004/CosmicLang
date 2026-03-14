// Step 3

// So, the evaluator walks through the AST and computes the final value
// It als holds a HashMap of variables names -> values

use crate::parser_folder::parser::{Expr, Op};
use crate::value::Value;
use crate::stmt::{Stmt};
use std::collections::HashMap;

pub struct Evaluator {
    // the Hashmap maps the variable names to their values
    // for now we have only one scope that is GLOBAL
    env: HashMap<String, Value>, // this stores and returns Value
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: HashMap::new(), // Assigning new HashMap
        }
    }

    pub fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Expression(expr) => self.eval(expr),

            Stmt::Publish(expr) => {
                let val = self.eval(expr)?;
                println!("{}", val); // Value has Display now so this works
                Ok(Value::Nil)
            }
        }
    }

    // So, the main method will evaluate the node of Expr AST recursively
    // Returns: Result<f64, String>
    //              Ok(number): sucess
    //              Err(message): something went wrong
    pub fn eval(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            // Only number - just returning it
            Expr::Number(n) => Ok(Value::Number(n)),

            // A variable - check if present in the environment
            Expr::Variable(name) => {
                match self.env.get(&name) {
                    Some(val) => Ok(*val), // found it return the value, here derefrending is done because we promised to return a f64 not a &f64
                    None => Err(format!("Undefined variable {}", name)),
                }
            }

            // An assignment: x = expr
            // So, first evaluate the right then store it then return it
            Expr::Assign(name, value_expr) => {
                let val = self.eval(*value_expr)?; // recurursively solving the nested statements. Derefrencing because eval takes Expr not Box<Expr>
                self.env.insert(name.clone(), val);
                // println!("{} = {}", name, val); // display what was stored
                Ok(val)
            }

            // A binary operation: left Op right
            Expr::BinOp(left, op, right) => {
                let l = self.eval(*left)?; // evaluate the left subtree
                let r = self.eval(*right)?; // evaluate the right subtree 

                // Extracting numbers first
                let (l, r) = match (l, r) {
                    (Value::Number(a), Value::Number(b)) => (a, b), // is it's a match than extract in (a, b)
                    _ => return Err("Math operation required numbers".to_string()),
                };

                match op {
                    Op::Add => Ok(Value::Number(l + r)),
                    Op::Sub => Ok(Value::Number(l - r)),
                    Op::Mul => Ok(Value::Number(l * r)),
                    Op::Div => {
                        if r == 0.0 {
                            Err("Zero division error".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    Op::GreaterThan => Ok(Value::Bool(l > r)),
                    Op::LessThan => Ok(Value::Bool(l < r)),
                    Op::EqualEqual => Ok(Value::Bool(l == r)),
                    Op::NotEqualTo => Ok(Value::Bool(l != r)),
                }
            }
        }
    }

    // A getter method to see what variables are stored
    pub fn get_env(&self) -> &HashMap<String, Value> {
        &self.env
    }
}
