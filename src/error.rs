use std::error;
use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum InterpreterError {
    InvalidSyntax(String),
    UnexpectedToken(Token),
    UndefinedSymbol(String),
    RedefineBuiltinSymbol(String),
    EOF,
}

impl error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::InvalidSyntax(syntax) => {
                write!(f, "Invalid syntax: {}", syntax)
            }
            InterpreterError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
            InterpreterError::UndefinedSymbol(symbol) => {
                write!(f, "Undefined symbol: {}", symbol)
            }
            InterpreterError::RedefineBuiltinSymbol(symbol) => {
                write!(f, "Redefine builtin symbol: {}", symbol)
            }
            InterpreterError::EOF => {
                write!(f, "End of file.")
            }
        }
    }
}
