use std::collections::HashMap;

use crate::error::InterpreterError;

pub trait ASTNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError>;
}
