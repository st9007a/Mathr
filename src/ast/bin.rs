use super::ast::{ASTNode, BinaryOpASTNode};

pub struct AddExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    left: S,
    right: T,
}

pub struct SubExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    left: S,
    right: T,
}

pub struct MulExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    left: S,
    right: T,
}

pub struct DivExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    left: S,
    right: T,
}

impl<S, T> AddExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    pub fn new(left: S, right: T) -> Self {
        Self { left, right }
    }
}

impl<S, T> BinaryOpASTNode for AddExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn op(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}

impl<S, T> ASTNode for AddExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn eval(&self) -> u32 {
        self.op(self.left.eval(), self.right.eval())
    }
}

impl<S, T> SubExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    pub fn new(left: S, right: T) -> Self {
        Self { left, right }
    }
}

impl<S, T> BinaryOpASTNode for SubExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn op(&self, a: u32, b: u32) -> u32 {
        a - b
    }
}

impl<S, T> ASTNode for SubExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn eval(&self) -> u32 {
        self.op(self.left.eval(), self.right.eval())
    }
}

impl<S, T> MulExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    pub fn new(left: S, right: T) -> Self {
        Self { left, right }
    }
}

impl<S, T> BinaryOpASTNode for MulExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn op(&self, a: u32, b: u32) -> u32 {
        a * b
    }
}

impl<S, T> ASTNode for MulExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn eval(&self) -> u32 {
        self.op(self.left.eval(), self.right.eval())
    }
}

impl<S, T> DivExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    pub fn new(left: S, right: T) -> Self {
        Self { left, right }
    }
}

impl<S, T> BinaryOpASTNode for DivExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn op(&self, a: u32, b: u32) -> u32 {
        a / b
    }
}

impl<S, T> ASTNode for DivExprNode<S, T>
where
    S: ASTNode,
    T: ASTNode,
{
    fn eval(&self) -> u32 {
        self.op(self.left.eval(), self.right.eval())
    }
}
