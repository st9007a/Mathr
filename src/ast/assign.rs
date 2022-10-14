use std::collections::HashMap;

use crate::error::InterpreterError;

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
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let value = self.expr.eval(symtab)?;

        symtab.insert(self.var.get_name().to_string(), value);

        Ok(value)
    }
}
