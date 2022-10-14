use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ast::{ASTNode, BinaryOpFunction};

pub struct AddNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl AddNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl BinaryOpFunction for AddNode {
    fn exec(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

impl ASTNode for AddNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, InterpreterError> {
        let left = self.left.eval(symtab)?;
        let right = self.right.eval(symtab)?;

        Ok(self.exec(left, right))
    }
}
