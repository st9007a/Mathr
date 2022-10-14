use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ast::ASTNode;

pub struct NumberNode {
    value: i32,
}

impl NumberNode {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl ASTNode for NumberNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, InterpreterError> {
        Ok(self.value)
    }
}
