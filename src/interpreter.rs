use std::collections::HashMap;

use crate::ast::StatementListNode;

pub struct Interpreter {
    global: HashMap<String, i32>,
    nodes: Vec<Box<StatementListNode>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut global: HashMap<String, i32> = HashMap::new();

        global.insert("e".to_string(), 2);
        global.insert("pi".to_string(), 3);

        Self {
            global,
            nodes: vec![],
        }
    }
}
