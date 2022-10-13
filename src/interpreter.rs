use std::collections::HashMap;

use crate::ast::StatementListNode;

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
}
