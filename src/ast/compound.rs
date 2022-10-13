use super::ASTNode;

pub struct CompoundNode {
    nodes: Vec<Box<dyn ASTNode>>,
}

impl CompoundNode {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from_nodes(nodes: Vec<Box<dyn ASTNode>>) -> Self {
        Self { nodes }
    }
}

impl ASTNode for CompoundNode {
    fn eval(&self) -> i32 {
        panic!("Cannot evaluate this node !");
    }
}
