use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode};

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTNode for NumberNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        self.eval(symtab)
    }
}

impl ASTExpression for NumberNode {
    fn pure(&self) -> bool {
        true
    }

    fn eval(&self, _symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        Ok(self.value)
    }

    fn check_symbol(&self, symtab: &SymbolTable) -> Result<(), InterpreterError> {
        Ok(())
    }
}
