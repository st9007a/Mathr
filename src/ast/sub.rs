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
    fn exec(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

impl ASTNode for SubNode {
    fn eval(&self) -> i32 {
        self.exec(self.left.eval(), self.right.eval())
    }
}
