use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

pub trait ASTNode {}

pub trait ASTStatement {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError>;
}

pub trait ASTExpression {
    fn pure(&self) -> bool;

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError>;
}

pub trait ASTSemanticAnalysis {
    fn check_semantic(&self, symtab: &mut SymbolTable) -> Result<(), InterpreterError>;
}

pub trait ASTSemanticStatement: ASTNode + ASTStatement + ASTSemanticAnalysis {}

pub trait ASTSemanticExpression: ASTNode + ASTExpression + ASTSemanticAnalysis {}

impl<T: ASTNode + ASTExpression + ASTSemanticAnalysis> ASTSemanticExpression for T {}
impl<T: ASTNode + ASTStatement + ASTSemanticAnalysis> ASTSemanticStatement for T {}
