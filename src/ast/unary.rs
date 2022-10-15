use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode};

pub enum UnaryOpType {
    PLUS,
    MINUS,
}

pub struct UnaryOpNode {
    node: Box<dyn ASTExpression>,
    op_type: UnaryOpType,
}

impl UnaryOpNode {
    pub fn new(node: Box<dyn ASTExpression>, op_type: UnaryOpType) -> Self {
        Self { node, op_type }
    }
}

impl ASTNode for UnaryOpNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        self.eval(symtab)
    }
}

impl ASTExpression for UnaryOpNode {
    fn pure(&self) -> bool {
        self.node.pure()
    }

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        let value = self.node.eval(symtab)?;

        match self.op_type {
            UnaryOpType::PLUS => Ok(value),
            UnaryOpType::MINUS => Ok(-value),
        }
    }

    fn check_symbol(&self, symtab: &SymbolTable) -> Result<(), InterpreterError> {
        if self.node.pure() {
            Ok(())
        } else {
            self.check_symbol(symtab)
        }
    }
}
