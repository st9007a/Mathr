use std::collections::HashMap;

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
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        let value = self.expr.eval(symtab);

        symtab.insert(self.var.get_name(), value);

        value
    }
}
