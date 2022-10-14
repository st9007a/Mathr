use std::error;
use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum InterpreterError {
    InvalidSyntax(String),
    UnexpectedToken(Token),
    UndefinedSymbol(String),
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
            InterpreterError::EOF => {
                write!(f, "End of file.")
            }
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedTokenError {
    token: Token,
}

impl UnexpectedTokenError {
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl error::Error for UnexpectedTokenError {}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected token: {:?}", self.token)
    }
}
