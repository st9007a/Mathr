use std::collections::HashMap;

use super::ast::{ASTNode, BinaryOpFunction};
use super::error::UndefinedSymbolError;

pub struct DivNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl DivNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl BinaryOpFunction for DivNode {
    fn exec(&self, a: i32, b: i32) -> i32 {
        a / b
    }
}

impl ASTNode for DivNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, UndefinedSymbolError> {
        let left = self.left.eval(symtab)?;
        let right = self.right.eval(symtab)?;

        Ok(self.exec(left, right))
    }
}
