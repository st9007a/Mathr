use std::collections::HashMap;

use super::error::UndefinedSymbolError;
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
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, UndefinedSymbolError> {
        let mut value: i32 = 0;

        for node in self.nodes.iter() {
            value = node.eval(symtab)?;
        }

        Ok(value)
    }
}
