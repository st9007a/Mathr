use std::collections::HashMap;

use super::ast::{ASTNode, UnaryOpFunction};

pub struct PosNode {
    node: Box<dyn ASTNode>,
}

impl PosNode {
    pub fn new(node: Box<dyn ASTNode>) -> Self {
        Self { node }
    }
}

impl UnaryOpFunction for PosNode {
    fn exec(&self, a: i32) -> i32 {
        a
    }
}

impl ASTNode for PosNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        self.exec(self.node.eval(symtab))
    }
}
