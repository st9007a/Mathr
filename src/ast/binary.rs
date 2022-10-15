use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ASTExpression;

pub enum BinaryOpType {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub struct BinaryOpNode {
    left: Box<dyn ASTExpression>,
    right: Box<dyn ASTExpression>,
    op_type: BinaryOpType,
}

impl BinaryOpNode {
    pub fn new(
        left: Box<dyn ASTExpression>,
        right: Box<dyn ASTExpression>,
        op_type: BinaryOpType,
    ) -> Self {
        Self {
            left,
            right,
            op_type,
        }
    }
}

impl ASTExpression for BinaryOpNode {
    fn pure(&self) -> bool {
        self.left.pure() && self.right.pure()
    }

    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let left = self.left.eval(symtab)?;
        let right = self.right.eval(symtab)?;

        match self.op_type {
            BinaryOpType::ADD => Ok(left + right),
            BinaryOpType::SUB => Ok(left - right),
            BinaryOpType::MUL => Ok(left * right),
            BinaryOpType::DIV => Ok(left / right),
        }
    }
}
