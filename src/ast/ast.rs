use std::collections::HashMap;

use super::error::UndefinedSymbolError;

pub trait ASTNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, UndefinedSymbolError>;
}

pub trait UnaryOpFunction {
    fn exec(&self, a: i32) -> i32;
}

pub trait BinaryOpFunction {
    fn exec(&self, a: i32, b: i32) -> i32;
}
