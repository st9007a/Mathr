use crate::error::InterpreterError;
use crate::symbol_table::SymbolTable;

pub trait ASTNode {
    fn execute(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError>;
}

pub trait ASTExpression {
    fn pure(&self) -> bool;

    fn eval(&self, symtab: &mut SymbolTable) -> Result<f64, InterpreterError>;

    fn check_symbol(&self, symtab: &SymbolTable) -> Result<(), InterpreterError>;
}
