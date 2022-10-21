use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

use super::{ASTExpression, ASTNode, ASTSemanticAnalysis};

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl ASTNode for VarNode {}

impl ASTExpression for VarNode {
    fn pure(&self) -> bool {
        false
    }

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError> {
        symtab
            .get(self.name())
            .map(|value| *value)
            .ok_or(InterpreterError::UndefinedSymbol(self.name().clone()))
    }
}

impl ASTSemanticAnalysis for VarNode {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError> {
        symtab
            .get(self.name())
            .map(|_| ())
            .ok_or(InterpreterError::UndefinedSymbol(self.name().clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::symbol_table::SymbolTable;

    use super::ASTExpression;
    use super::ASTSemanticAnalysis;
    use super::VarNode;

    #[test]
    fn test_eval() {
        let mut symtab = SymbolTable::new();
        let node = VarNode::new("x".to_string());
    
        symtab.insert("x".to_string(), 25.);

        let result = node.eval(&mut symtab);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 25.);
    }

    #[test]
    fn test_eval_err() {
        let mut symtab = SymbolTable::new();
        let node = VarNode::new("x".to_string());
        let result = node.eval(&mut symtab);

        assert!(result.is_err());
    }

    #[test]
    fn test_pure() {
        let node = VarNode::new("x".to_string());
        assert!(!node.pure());
    }

    #[test]
    fn test_check_semantic() {
        let mut symtab = SymbolTable::new();
        let node = VarNode::new("x".to_string());

        symtab.insert("x".to_string(), 23.);

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_semantic_err() {
        let mut symtab = SymbolTable::new();
        let node = VarNode::new("x".to_string());

        let result = node.check_semantic(&mut symtab);

        assert!(result.is_err());
    }
}
