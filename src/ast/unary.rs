use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ASTExpression;

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

impl ASTExpression for UnaryOpNode {
    fn pure(&self) -> bool {
        self.node.pure()
    }

    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let value = self.node.eval(symtab)?;

        match self.op_type {
            UnaryOpType::PLUS => Ok(value),
            UnaryOpType::MINUS => Ok(-value),
        }
    }
}
