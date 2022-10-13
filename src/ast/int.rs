use std::collections::HashMap;

use super::ast::ASTNode;

pub struct IntegerNode {
    value: i32,
}

impl IntegerNode {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl ASTNode for IntegerNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        self.value
    }
}
