use std::collections::HashMap;

use super::{error::UndefinedSymbolError, ASTNode};

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> String {
        self.name
    }
}

impl ASTNode for VarNode {
    fn eval(&self, symtab: &mut HashMap<String, i32>) -> Result<i32, UndefinedSymbolError> {
        symtab
            .get(&self.name)
            .map(|value| *value)
            .ok_or(UndefinedSymbolError::new(self.name))
    }
}
