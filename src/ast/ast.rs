use std::collections::HashMap;

use crate::error::InterpreterError;

pub trait ASTNode {
    fn execute(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError>;
}

pub trait ASTExpression {
    fn pure(&self) -> bool;

    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError>;
}
