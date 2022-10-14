use std::collections::HashMap;
use std::f64::consts;

use crate::ast::{ASTNode, StatementListNode};
use crate::error::InterpreterError;
use crate::parser::Parser;

pub struct Interpreter {
    symtab: HashMap<String, f64>,
    nodes: Vec<Box<StatementListNode>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut symtab: HashMap<String, f64> = HashMap::new();

        symtab.insert("e".to_string(), consts::E);
        symtab.insert("pi".to_string(), consts::PI);

        Self {
            symtab,
            nodes: vec![],
        }
    }

    pub fn interpret(&mut self, content: &str) -> Result<f64, InterpreterError> {
        let mut parser = Parser::from_text(content);
        let statement_list_node = parser.parse()?;

        let value = statement_list_node.eval(&mut self.symtab)?;
        self.nodes.push(statement_list_node);

        Ok(value)
    }

    pub fn clear_state(&mut self) {
        self.nodes.clear();
        self.symtab.clear();
        self.symtab.insert("e".to_string(), consts::E);
        self.symtab.insert("pi".to_string(), consts::PI);
    }

    pub fn query(&self, symbol: &String) -> Option<&f64> {
        self.symtab.get(symbol)
    }
}
