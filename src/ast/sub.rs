use super::ast::{ASTNode, BinaryOpFunction};

pub struct SubNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl SubNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl BinaryOpFunction for SubNode {
    fn exec(&self, a: u32, b: u32) -> u32 {
        a - b
    }
}

impl ASTNode for SubNode {
    fn eval(&self) -> u32 {
        self.exec(self.left.eval(), self.right.eval())
    }
}
