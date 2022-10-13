use super::{ASTNode, AssignNode};

pub struct StatementListNode {
    nodes: Vec<Box<AssignNode>>,
}

impl StatementListNode {
    pub fn new(nodes: Vec<Box<AssignNode>>) -> Self { Self { nodes } }
}

impl ASTNode for StatementListNode {
    fn eval(&self) -> i32 {
        panic!("Cannot be evaluated !");
    }
}
