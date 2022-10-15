use std::collections::HashMap;
use std::f64::consts;

pub struct SymbolTable {
    global: HashMap<String, f64>,
    local: HashMap<String, f64>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut global: HashMap<String, f64> = HashMap::new();

        global.insert("e".to_string(), consts::E);
        global.insert("pi".to_string(), consts::PI);

        Self {
            global,
            local: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.local.clear();
    }

    pub fn get(&self, symbol: &String) -> Option<&f64> {
        self.global.get(symbol).or(self.local.get(symbol))
    }

    pub fn insert(&mut self, symbol: String, value: f64) {
        self.local.insert(symbol, value);
    }
}
