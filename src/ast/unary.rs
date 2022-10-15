use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis, ASTSemanticExpression};

pub enum UnaryOpType {
    PLUS,
    MINUS,
}

pub struct UnaryOpNode {
    node: Box<dyn ASTSemanticExpression>,
    op_type: UnaryOpType,
}

impl UnaryOpNode {
    pub fn new(node: Box<dyn ASTSemanticExpression>, op_type: UnaryOpType) -> Self {
        Self { node, op_type }
    }
}

impl ASTNode for UnaryOpNode {}

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
}

impl ASTSemanticAnalysis for UnaryOpNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        if self.node.pure() {
            Ok(())
        } else {
            self.check_semantic(symtab)
        }
    }
}
