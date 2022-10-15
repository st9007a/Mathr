use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTNode, ASTSemanticAnalysis, ASTSemanticExpression, ASTStatement, VarNode};

pub struct AssignNode {
    var: Box<VarNode>,
    expression: Box<dyn ASTSemanticExpression>,
}

impl AssignNode {
    pub fn new(var: Box<VarNode>, expression: Box<dyn ASTSemanticExpression>) -> Self {
        Self { var, expression }
    }
}

impl ASTStatement for AssignNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        let value = self.expression.eval(symtab)?;

        symtab.insert(self.var.name().to_string(), value);

        Ok(value)
    }
}

impl ASTNode for AssignNode {}

impl ASTSemanticAnalysis for AssignNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        if !self.expression.pure() {
            let res = self.expression.check_semantic(symtab);

            if res.is_err() {
                return res;
            }
        }

        if symtab.is_global(self.var.name()) {
            Err(InterpreterError::RedefineBuiltinSymbol(
                self.var.name().clone(),
            ))
        } else {
            symtab.insert(self.var.name().clone(), 0.);
            Ok(())
        }
    }
}
