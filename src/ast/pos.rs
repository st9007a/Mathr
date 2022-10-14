use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ast::{ASTNode, UnaryOpFunction};

pub struct PosNode {
    node: Box<dyn ASTNode>,
}

impl PosNode {
    pub fn new(node: Box<dyn ASTNode>) -> Self {
        Self { node }
    }
}

impl UnaryOpFunction for PosNode {
    fn exec(&self, a: f64) -> f64 {
        a
    }
}

impl ASTNode for PosNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let value = self.node.eval(symtab)?;
        Ok(self.exec(value))
    }
}
