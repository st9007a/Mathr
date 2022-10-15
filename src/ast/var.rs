use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis};

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl ASTNode for VarNode {}

impl ASTExpression for VarNode {
    fn pure(&self) -> bool {
        false
    }

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        symtab
            .get(self.name())
            .map(|value| *value)
            .ok_or(InterpreterError::UndefinedSymbol(self.name().clone()))
    }
}

impl ASTSemanticAnalysis for VarNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        symtab
            .get(self.name())
            .map(|_| ())
            .ok_or(InterpreterError::UndefinedSymbol(self.name().clone()))
    }
}
