pub trait ASTNode {
    fn eval(&self) -> u32;
}

pub trait BinaryOpFunction {
    fn exec(&self, a: u32, b: u32) -> u32;
}
