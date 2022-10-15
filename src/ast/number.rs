use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ASTExpression;

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTExpression for NumberNode {
    fn pure(&self) -> bool {
        true
    }

    fn eval(&self, _symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        Ok(self.value)
    }
}
