use std::collections::HashMap;

use crate::error::InterpreterError;

pub trait ASTNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError>;
}

pub trait UnaryOpFunction {
    fn exec(&self, a: f64) -> f64;
}

pub trait BinaryOpFunction {
    fn exec(&self, a: f64, b: f64) -> f64;
}
