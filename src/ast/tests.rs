use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis};

pub struct MockNode {
    eval_value: f64,
    eval_err: InterpreterError,
    eval_is_ok: bool,
    pure: bool,
    check_semantic_err: InterpreterError,
    check_semantic_is_ok: bool,
}

impl MockNode {
    pub fn new() -> Self {
        Self {
            eval_value: 0.,
            eval_err: InterpreterError::EOF,
            eval_is_ok: true,
            pure: true,
            check_semantic_err: InterpreterError::EOF,
            check_semantic_is_ok: true,
        }
    }

    pub fn expect_eval(&self, expect_value: f64) -> Self {
        Self {
            eval_value: expect_value,
            eval_err: InterpreterError::EOF,
            eval_is_ok: true,
            pure: self.pure,
            check_semantic_err: self.check_semantic_err.clone(),
            check_semantic_is_ok: self.check_semantic_is_ok,
        }
    }

    pub fn expect_eval_err(&self, err: InterpreterError) -> Self {
        Self {
            eval_value: self.eval_value,
            eval_err: err,
            eval_is_ok: false,
            pure: self.pure,
            check_semantic_err: self.check_semantic_err.clone(),
            check_semantic_is_ok: self.check_semantic_is_ok,
        }
    }

    pub fn expect_pure(&self, expect_value: bool) -> Self {
        Self {
            eval_value: self.eval_value,
            eval_err: self.eval_err.clone(),
            eval_is_ok: self.eval_is_ok,
            pure: expect_value,
            check_semantic_err: self.check_semantic_err.clone(),
            check_semantic_is_ok: self.check_semantic_is_ok,
        }
    }

    pub fn expect_check_semantic(&self) -> Self {
        Self {
            eval_value: self.eval_value,
            eval_err: self.eval_err.clone(),
            eval_is_ok: self.eval_is_ok,
            pure: self.pure,
            check_semantic_err: self.check_semantic_err.clone(),
            check_semantic_is_ok: true,
        }
    }

    pub fn expect_check_semantic_err(&self, err: InterpreterError) -> Self {
        Self {
            eval_value: self.eval_value,
            eval_err: self.eval_err.clone(),
            eval_is_ok: self.eval_is_ok,
            pure: self.pure,
            check_semantic_err: err,
            check_semantic_is_ok: false,
        }
    }
}

impl ASTNode for MockNode {}

impl ASTExpression for MockNode {
    fn pure(&self) -> bool {
        self.pure
    }

    fn eval(&self, _symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        if self.eval_is_ok {
            Ok(self.eval_value)
        } else {
            Err(self.eval_err.clone())
        }
    }
}

impl ASTSemanticAnalysis for MockNode {
    fn check_semantic(&self, _symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        if self.check_semantic_is_ok {
            Ok(())
        } else {
            Err(self.check_semantic_err.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::InterpreterError;
    use crate::symbol_table::SymbolTable;

    use super::{ASTExpression, ASTSemanticAnalysis, MockNode};

    #[test]
    fn test_eval() {
        let mut symtab = SymbolTable::new();
        let node = MockNode::new().expect_eval(12.);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12.);
    }

    #[test]
    fn test_eval_err() {
        let mut symtab = SymbolTable::new();
        let node = MockNode::new().expect_eval_err(InterpreterError::EOF);
        let result = node.eval(&mut symtab);

        assert!(result.is_err());
    }

    #[test]
    fn test_pure() {
        let node1 = MockNode::new().expect_pure(false);
        let node2 = MockNode::new().expect_pure(true);

        assert!(!node1.pure());
        assert!(node2.pure());
    }

    #[test]
    fn test_check_semantic() {
        let mut symtab = SymbolTable::new();
        let node = MockNode::new().expect_check_semantic();

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_semantic_err() {
        let mut symtab = SymbolTable::new();
        let node = MockNode::new().expect_check_semantic_err(InterpreterError::EOF);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_err());
    }
}
