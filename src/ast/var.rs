use super::ASTNode;

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl ASTNode for VarNode {
    fn eval(&self) -> i32 {
        panic!("Cannot evaluate this node !");
    }
}
