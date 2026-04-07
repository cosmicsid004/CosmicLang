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

            Stmt::Block(stmts) => {
                let mut last = Value::Nil;
                for stmt in stmts {
                    last = self.eval_stmt(stmt)?;
                }
                Ok(last)
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
            Expr::StringLiteral(s) => Ok(Value::Str(s)),

            // A variable - check if present in the environment
            Expr::Variable(name) => {
                match self.env.get(&name) {
                    // switching *val to val.clone() in variable lookup (since String isn't Copy)
                    Some(val) => Ok(val.clone()), // found it return the value, here derefrending is done because we promised to return a f64 not a &f64
                    None => Err(format!("Undefined variable {}", name)),
                }
            }

            // An assignment: x = expr
            // So, first evaluate the right then store it then return it
            Expr::Assign(name, value_expr) => {
                let val = self.eval(*value_expr)?; // recurursively solving the nested statements. Derefrencing because eval takes Expr not Box<Expr>
                self.env.insert(name.clone(), val.clone()); //  switching *val to val.clone() in variable lookup (since String isn't Copy)
                // println!("{} = {}", name, val); // display what was stored
                Ok(val)
            }

            // A binary operation: left Op right
            Expr::BinOp(left, op, right) => {
                let l = self.eval(*left)?; // evaluate the left subtree
                let r = self.eval(*right)?; // evaluate the right subtree 

                // Handling string concatination
                if let (Value::Str(a), Value::Str(b)) = (&l, &r) {
                    if let Op::Add = op {
                        return Ok(Value::Str(format!("{}{}", a, b)))
                    }
                }

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
                    Op::GreaterThanEqual => Ok(Value::Bool(l >= r)),
                    Op::LessThanEqual => Ok(Value::Bool(l <= r))
                }
            }

            Expr::Ternary(condition, true_expr , false_expr) => {
                let cond = self.eval(*condition)?; // evalate the condition subtree
                match cond {
                    Value::Bool(true) => self.eval_stmt(*true_expr),
                    Value::Bool(false) => self.eval_stmt(*false_expr),
                    _ => Err("Ternary condition must return a boolean".to_string()),
                }
            }
        }
    }

    // A getter method to see what variables are stored
    pub fn get_env(&self) -> &HashMap<String, Value> {
        &self.env
    }
}
