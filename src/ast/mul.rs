use std::collections::HashMap;

use super::ast::{ASTNode, BinaryOpFunction};

pub struct MulNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl MulNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl BinaryOpFunction for MulNode {
    fn exec(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

impl ASTNode for MulNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        self.exec(self.left.eval(symtab), self.right.eval(symtab))
    }

    fn is_variable(&self) -> bool {
        false
    }
}
