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

    pub fn is_global(&self, symbol: &String) -> bool {
        self.global.get(symbol).is_some()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::SymbolTable;

    #[test]
    fn test_get() {
        let symtab = SymbolTable::new();

        let kv_pairs = vec![
            ("e".to_string(), Some(&consts::E)),
            ("pi".to_string(), Some(&consts::PI)),
            ("x".to_string(), None),
            ("test_key".to_string(), None),
        ];

        for kv in kv_pairs.iter() {
            assert_eq!(symtab.get(&kv.0), kv.1);
        }
    }

    #[test]
    fn test_insert() {
        let mut symtab = SymbolTable::new();

        symtab.insert("x".to_string(), 123.45);

        assert_eq!(symtab.get(&"x".to_string()), Some(&123.45));
    }

    #[test]
    fn test_is_global() {
        let symtab = SymbolTable::new();

        assert!(symtab.is_global(&"e".to_string()));
        assert!(symtab.is_global(&"pi".to_string()));
        assert!(!symtab.is_global(&"x".to_string()));
        assert!(!symtab.is_global(&"test_var".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut symtab = SymbolTable::new();

        symtab.insert("x".to_string(), 123.45);

        assert_eq!(symtab.get(&"x".to_string()), Some(&123.45));

        symtab.clear();

        assert_eq!(symtab.get(&"x".to_string()), None);
        assert_eq!(symtab.get(&"e".to_string()), Some(&consts::E));
        assert_eq!(symtab.get(&"pi".to_string()), Some(&consts::PI));
    }
}
