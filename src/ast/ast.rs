use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

pub trait ASTNode {
    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError>;
}

pub trait ASTExpression {
    fn pure(&self) -> bool;
}

pub trait ASTSemanticAnalysis {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError>;
}

pub trait ASTSemanticStatement: ASTNode + ASTSemanticAnalysis {}

pub trait ASTSemanticExpression: ASTNode + ASTExpression + ASTSemanticAnalysis {}

impl<T: ASTNode + ASTExpression + ASTSemanticAnalysis> ASTSemanticExpression for T {}
impl<T: ASTNode + ASTSemanticAnalysis> ASTSemanticStatement for T {}
