use std::collections::HashMap;

use crate::error::InterpreterError;

use super::{ASTNode, AssignNode};

pub struct StatementListNode {
    nodes: Vec<Box<AssignNode>>,
}

impl StatementListNode {
    pub fn new(nodes: Vec<Box<AssignNode>>) -> Self {
        Self { nodes }
    }
}

impl ASTNode for StatementListNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        let mut value: f64 = 0.;

        for node in self.nodes.iter() {
            value = node.eval(symtab)?;
        }

        Ok(value)
    }
}
