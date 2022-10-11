use std::error;
use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub struct InvalidSyntaxError {
    content: String,
}

impl InvalidSyntaxError {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl error::Error for InvalidSyntaxError {}

impl fmt::Display for InvalidSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid syntax: {}", self.content)
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
