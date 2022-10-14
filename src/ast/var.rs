use std::collections::HashMap;

use crate::error::InterpreterError;

use super::ASTNode;

pub struct VarNode {
    name: String,
}

impl VarNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl ASTNode for VarNode {
    fn eval(&self, symtab: &mut HashMap<String, f64>) -> Result<f64, InterpreterError> {
        symtab
            .get(&self.name)
            .map(|value| *value)
            .ok_or(InterpreterError::UndefinedSymbol(self.name.clone()))
    }
}
