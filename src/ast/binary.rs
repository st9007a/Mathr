use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode};

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

impl ASTNode for BinaryOpNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        self.eval(symtab)
    }
}

impl ASTExpression for BinaryOpNode {
    fn pure(&self) -> bool {
        self.left.pure() && self.right.pure()
    }

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        let left = self.left.eval(symtab)?;
        let right = self.right.eval(symtab)?;

        match self.op_type {
            BinaryOpType::ADD => Ok(left + right),
            BinaryOpType::SUB => Ok(left - right),
            BinaryOpType::MUL => Ok(left * right),
            BinaryOpType::DIV => Ok(left / right),
        }
    }

    fn check_symbol(&self, symtab: &SymbolTable) -> Result<(), InterpreterError> {
        let mut res = Ok(());

        if !self.left.pure() {
            res = res.and(self.left.check_symbol(symtab));
        }
        if !self.right.pure() {
            res = res.and(self.right.check_symbol(symtab));
        }

        res
    }
}
