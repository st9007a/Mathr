use super::ASTNode;

pub struct StatementListNode {
    nodes: Vec<Box<dyn ASTNode>>,
}

impl StatementListNode {
    pub fn new(nodes: Vec<Box<dyn ASTNode>>) -> Self { Self { nodes } }
}

impl ASTNode for StatementListNode {
    fn eval(&self) -> i32 {
        panic!("Cannot be evaluated !");
    }
}
