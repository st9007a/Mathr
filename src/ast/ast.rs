pub trait ASTNode {
    fn eval(&self) -> u32;
}

pub trait BinaryOpASTNode {
    fn op(&self, a: u32, b: u32) -> u32;
}
