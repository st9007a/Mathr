use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, VarNode};

pub struct AssignNode {
    var: Box<VarNode>,
    expression: Box<dyn ASTExpression>,
}

impl AssignNode {
    pub fn new(var: Box<VarNode>, expression: Box<dyn ASTExpression>) -> Self {
        Self { var, expression }
    }

    pub fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        if !self.expression.pure() {
            let res = self.expression.check_symbol(symtab);

            if res.is_err() {
                return res;
            }
        }

        if symtab.is_global(self.var.name()) {
            Err(InterpreterError::RedefineBuiltinSymbol(
                self.var.name().clone(),
            ))
        } else {
            Ok(())
        }
    }
}

impl ASTNode for AssignNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        let value = self.expression.eval(symtab)?;

        symtab.insert(self.var.name().to_string(), value);

        Ok(value)
    }
}
