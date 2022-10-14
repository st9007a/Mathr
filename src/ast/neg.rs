use std::collections::HashMap;

use super::ast::{ASTNode, UnaryOpFunction};
use crate::error::InterpreterError;

pub struct NegNode {
    node: Box<dyn ASTNode>,
}

impl NegNode {
    pub fn new(node: Box<dyn ASTNode>) -> Self {
        Self { node }
    }
}

impl UnaryOpFunction for NegNode {
    fn exec(&self, a: f64) -> f64 {
        -a
    }
}

impl ASTNode for NegNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let value = self.node.eval(symtab)?;
        Ok(self.exec(value))
    }
}
