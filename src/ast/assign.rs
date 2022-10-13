use super::{ASTNode, VarNode};

pub struct AssignNode {
    var: Box<VarNode>,
    expr: Box<dyn ASTNode>,
}

impl AssignNode {
    pub fn new(var: Box<VarNode>, expr: Box<dyn ASTNode>) -> Self {
        Self { var, expr }
    }
}

impl ASTNode for AssignNode {
    fn eval(&self) -> i32 {
        panic!("Cannot evalute this node !");
    }
}
