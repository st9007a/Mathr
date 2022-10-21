use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis, ASTSemanticExpression};

pub enum BinaryOpType {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub struct BinaryOpNode {
    left: Box<dyn ASTSemanticExpression>,
    right: Box<dyn ASTSemanticExpression>,
    op_type: BinaryOpType,
}

impl BinaryOpNode {
    pub fn new(
        left: Box<dyn ASTSemanticExpression>,
        right: Box<dyn ASTSemanticExpression>,
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
}

impl ASTExpression for BinaryOpNode {
    fn pure(&self) -> bool {
        self.left.pure() && self.right.pure()
    }
}

impl ASTSemanticAnalysis for BinaryOpNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        let mut res = Ok(());

        if !self.left.pure() {
            res = res.and(self.left.check_semantic(symtab));
        }
        if !self.right.pure() {
            res = res.and(self.right.check_semantic(symtab));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::tests::MockNode;
    use crate::error::InterpreterError;
    use crate::symbol_table::SymbolTable;

    use super::{ASTExpression, ASTNode, ASTSemanticAnalysis, BinaryOpNode, BinaryOpType};

    #[test]
    fn test_eval_add() {
        let lvalue: f64 = 32.;
        let rvalue: f64 = 128.;
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_eval(lvalue);
        let right = MockNode::new().expect_eval(rvalue);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), lvalue + rvalue);
    }

    #[test]
    fn test_eval_sub() {
        let lvalue: f64 = 32.;
        let rvalue: f64 = 128.;
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_eval(lvalue);
        let right = MockNode::new().expect_eval(rvalue);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::SUB);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), lvalue - rvalue);
    }

    #[test]
    fn test_eval_mul() {
        let lvalue: f64 = 32.;
        let rvalue: f64 = 128.;
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_eval(lvalue);
        let right = MockNode::new().expect_eval(rvalue);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::MUL);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), lvalue * rvalue);
    }

    #[test]
    fn test_eval_div() {
        let lvalue: f64 = 32.;
        let rvalue: f64 = 128.;
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_eval(lvalue);
        let right = MockNode::new().expect_eval(rvalue);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::DIV);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), lvalue / rvalue);
    }

    #[test]
    fn test_pure() {
        let left = MockNode::new().expect_pure(true);
        let right = MockNode::new().expect_pure(true);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        assert!(node.pure());
    }

    #[test]
    fn test_pure_left_is_false() {
        let left = MockNode::new().expect_pure(false);
        let right = MockNode::new().expect_pure(true);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        assert!(!node.pure());
    }

    #[test]
    fn test_pure_right_is_false() {
        let left = MockNode::new().expect_pure(true);
        let right = MockNode::new().expect_pure(false);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        assert!(!node.pure());
    }

    #[test]
    fn test_check_semantic_all_pure() {
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_pure(true);
        let right = MockNode::new().expect_pure(true);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_semantic_not_pure() {
        let mut symtab = SymbolTable::new();
        let left = MockNode::new().expect_pure(false).expect_check_semantic();
        let right = MockNode::new().expect_pure(true);
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_semantic_err() {
        let mut symtab = SymbolTable::new();
        let left = MockNode::new()
            .expect_pure(false)
            .expect_check_semantic_err(InterpreterError::EOF);
        let right = MockNode::new().expect_pure(false).expect_check_semantic();
        let node = BinaryOpNode::new(Box::new(left), Box::new(right), BinaryOpType::ADD);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_err());
    }
}
