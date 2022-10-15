use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis};

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTNode for NumberNode {}

impl ASTExpression for NumberNode {
    fn pure(&self) -> bool {
        true
    }

    fn eval(&self, _symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        Ok(self.value)
    }
}

impl ASTSemanticAnalysis for NumberNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        Ok(())
    }
}
