use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis};

pub struct NumberNode {
    value: f64,
}

impl NumberNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl ASTNode for NumberNode {}

impl ASTExpression for NumberNode {
    fn pure(&self) -> bool {
        true
    }

    fn eval(&self, _symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        Ok(self.value)
    }
}

impl ASTSemanticAnalysis for NumberNode {
    fn check_semantic(&self, _symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::SymbolTable;

    use super::ASTExpression;
    use super::ASTSemanticAnalysis;
    use super::NumberNode;

    #[test]
    fn test_eval() {
        let mut symtab = SymbolTable::new();
        let node = NumberNode::new(23.);
        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 23.);
    }

    #[test]
    fn test_pure() {
        let node = NumberNode::new(2.31);
        assert!(node.pure());
    }

    #[test]
    fn test_check_semantic() {
        let mut symtab = SymbolTable::new();
        let node = NumberNode::new(0.1234);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }
}
