use super::ast::{ASTNode, BinaryOpFunction};

pub struct AddNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl AddNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl BinaryOpFunction for AddNode {
    fn exec(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}

impl ASTNode for AddNode {
    fn eval(&self) -> u32 {
        self.exec(self.left.eval(), self.right.eval())
    }
}
