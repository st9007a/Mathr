use std::collections::HashMap;

use crate::ast::{ASTNode, StatementListNode};
use crate::error::InterpreterError;
use crate::parser::Parser;

pub struct Interpreter {
    symtab: HashMap<String, i32>,
    nodes: Vec<Box<StatementListNode>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut symtab: HashMap<String, i32> = HashMap::new();

        symtab.insert("e".to_string(), 2);
        symtab.insert("pi".to_string(), 3);

        Self {
            symtab,
            nodes: vec![],
        }
    }

    pub fn interpret(&mut self, content: &str) -> Result<(), InterpreterError> {
        let mut parser = Parser::from_text(content);
        let statement_list_node = parser.parse()?;

        statement_list_node.eval(&mut self.symtab)?;
        self.nodes.push(statement_list_node);

        Ok(())
    }

    pub fn clear_state(&mut self) {
        self.nodes.clear();
        self.symtab.clear();
        self.symtab.insert("e".to_string(), 2);
        self.symtab.insert("pi".to_string(), 3);
    }

    pub fn query(&self, symbol: &String) -> Option<&i32> {
        self.symtab.get(symbol)
    }
}
