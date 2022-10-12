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
    fn exec(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

impl ASTNode for AddNode {
    fn eval(&self) -> i32 {
        self.exec(self.left.eval(), self.right.eval())
    }
}
