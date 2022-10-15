use std::collections::HashMap;

use crate::error::InterpreterError;

use super::{ASTExpression, ASTNode, VarNode};

pub struct AssignNode {
    var: Box<VarNode>,
    expression: Box<dyn ASTExpression>,
}

impl AssignNode {
    pub fn new(var: Box<VarNode>, expression: Box<dyn ASTExpression>) -> Self {
        Self { var, expression }
    }
}

impl ASTNode for AssignNode {
    fn execute(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let value = self.expression.eval(symtab)?;

        symtab.insert(self.var.name().to_string(), value);

        Ok(value)
    }
}
