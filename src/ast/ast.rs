use std::collections::HashMap;

pub trait ASTNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32;
}

pub trait UnaryOpFunction {
    fn exec(&self, a: i32) -> i32;
}

pub trait BinaryOpFunction {
    fn exec(&self, a: i32, b: i32) -> i32;
}
