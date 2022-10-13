use std::collections::HashMap;

use super::ASTNode;

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl ASTNode for VarNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> i32 {
        *symtab.get(&self.name).expect("Undefined symbol")
    }
}
