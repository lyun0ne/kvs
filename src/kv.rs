use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
pub struct KvStore {
    element: HashMap<String, String>,
}

impl KvStore {
    /// creat a kv
    pub fn new() -> Self {
        KvStore {
            element: HashMap::new(),
        }
    }
    /// get value
    pub fn get(&self, key: String) -> Option<String> {
        self.element.get(&key).cloned()
    }
    /// set valuue
    pub fn set(&mut self, key: String, value: String) {
        self.element.insert(key, value);
    }
    /// remove a value
    pub fn remove(&mut self, key: String) {
        self.element.remove(&key);
    }
}
