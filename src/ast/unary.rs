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
            self.node.check_semantic(symtab)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::tests::MockNode;
    use crate::error::InterpreterError;
    use crate::symbol_table::SymbolTable;

    use super::{ASTExpression, ASTSemanticAnalysis, UnaryOpNode, UnaryOpType};

    #[test]
    fn test_eval_plus() {
        let value: f64 = 32.;
        let mut symtab = SymbolTable::new();
        let inner = MockNode::new().expect_eval(value).expect_pure(true);
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::PLUS);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);
    }

    #[test]
    fn test_eval_minus() {
        let value: f64 = 32.;
        let mut symtab = SymbolTable::new();
        let inner = MockNode::new().expect_eval(value).expect_pure(true);
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::MINUS);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -value);
    }

    #[test]
    fn test_pure() {
        let inner = MockNode::new().expect_pure(true);
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::PLUS);

        assert!(node.pure());
    }

    #[test]
    fn test_pure_is_false() {
        let inner = MockNode::new().expect_pure(false);
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::PLUS);

        assert!(!node.pure());
    }

    #[test]
    fn test_check_semantic() {
        let mut symtab = SymbolTable::new();
        let inner = MockNode::new().expect_pure(true).expect_check_semantic();
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::PLUS);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_semantic_err() {
        let mut symtab = SymbolTable::new();
        let inner = MockNode::new()
            .expect_pure(false)
            .expect_check_semantic_err(InterpreterError::EOF);
        let node = UnaryOpNode::new(Box::new(inner), UnaryOpType::PLUS);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_err());
    }
}
