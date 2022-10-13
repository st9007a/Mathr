use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct UndefinedSymbolError {
    symbol: String,
}

impl UndefinedSymbolError {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

impl error::Error for UndefinedSymbolError {}

impl fmt::Display for UndefinedSymbolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The symbol {} is undefined.", self.symbol)
    }
}
