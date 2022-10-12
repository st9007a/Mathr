pub trait ASTNode {
    fn eval(&self) -> i32;
}

pub trait BinaryOpFunction {
    fn exec(&self, a: i32, b: i32) -> i32;
}
