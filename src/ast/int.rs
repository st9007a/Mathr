use super::ast::ASTNode;

pub struct IntegerNode {
    value: u32,
}

impl IntegerNode {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl ASTNode for IntegerNode {
    fn eval(&self) -> u32 {
        self.value
    }
}
