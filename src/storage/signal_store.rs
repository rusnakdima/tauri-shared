use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_store_set_get() {
        let store = SignalStore::new();
        store.set("key", serde_json::json!("value"));
        assert_eq!(store.get("key"), Some(serde_json::json!("value")));
    }

    #[test]
    fn test_signal_store_update() {
        let store = SignalStore::new();
        store.set("counter", serde_json::json!(0));
        store.update("counter", |v| serde_json::json!(v.as_i64().unwrap() + 1));
        assert_eq!(store.get("counter"), Some(serde_json::json!(1)));
    }

    #[test]
    fn test_signal_store_delete() {
        let store = SignalStore::new();
        store.set("key", serde_json::json!("value"));
        store.delete("key");
        assert_eq!(store.get("key"), None);
    }
}

pub struct SignalStore {
    data: RwLock<HashMap<String, Value>>,
    subscribers: RwLock<Vec<Box<dyn Fn(&str, &Value) + Send + Sync>>>,
}

impl SignalStore {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub fn set(&self, key: &str, value: Value) {
        {
            let mut data = self.data.write().unwrap();
            data.insert(key.to_string(), value.clone());
        }
        self.notify(key, &value);
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let data = self.data.read().unwrap();
        data.get(key).cloned()
    }

    pub fn update<F>(&self, key: &str, f: F)
    where
        F: FnOnce(&Value) -> Value,
    {
        let value = {
            let data = self.data.read().unwrap();
            data.get(key).cloned()
        };
        if let Some(v) = value {
            let new_value = f(&v);
            self.set(key, new_value);
        }
    }

    pub fn delete(&self, key: &str) {
        let mut data = self.data.write().unwrap();
        data.remove(key);
    }

    pub fn keys(&self) -> Vec<String> {
        let data = self.data.read().unwrap();
        data.keys().cloned().collect()
    }

    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&str, &Value) + Send + Sync + 'static,
    {
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.push(Box::new(callback));
    }

    fn notify(&self, key: &str, value: &Value) {
        let subscribers = self.subscribers.read().unwrap();
        for subscriber in subscribers.iter() {
            subscriber(key, value);
        }
    }

    pub fn to_json(&self) -> Value {
        let data = self.data.read().unwrap();
        Value::Object(data.clone().into_iter().collect())
    }

    pub fn from_json(&self, json: Value) {
        if let Value::Object(map) = json {
            let mut data = self.data.write().unwrap();
            *data = map.into_iter().collect();
        }
    }
}

impl Default for SignalStore {
    fn default() -> Self {
        Self::new()
    }
}
