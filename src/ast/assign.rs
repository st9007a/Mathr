use super::ASTNode;

pub struct AssignNode {
    left: Box<dyn ASTNode>,
    right: Box<dyn ASTNode>,
}

impl AssignNode {
    pub fn new(left: Box<dyn ASTNode>, right: Box<dyn ASTNode>) -> Self {
        Self { left, right }
    }
}

impl ASTNode for AssignNode {
    fn eval(&self) -> i32 {
        panic!("Cannot evalute this node !");
    }
}
