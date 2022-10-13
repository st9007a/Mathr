use std::collections::HashMap;

use super::ast::{ASTNode, UnaryOpFunction};

pub struct NegNode {
    node: Box<dyn ASTNode>,
}

impl NegNode {
    pub fn new(node: Box<dyn ASTNode>) -> Self {
        Self { node }
    }
}

impl UnaryOpFunction for NegNode {
    fn exec(&self, a: i32) -> i32 {
        -a
    }
}

impl ASTNode for NegNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        self.exec(self.node.eval(symtab))
    }
}
