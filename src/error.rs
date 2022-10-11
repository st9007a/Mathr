use std::error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidTokenError;

impl InvalidTokenError {
    pub fn new() -> Self {
        Self {}
    }
}

impl error::Error for InvalidTokenError {}

impl fmt::Display for InvalidTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid token")
    }
}
