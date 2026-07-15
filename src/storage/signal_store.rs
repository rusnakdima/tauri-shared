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

  #[test]
  fn test_signal_store_edge_values() {
    let store = SignalStore::new();
    // Null value
    store.set("null_key", serde_json::Value::Null);
    assert_eq!(store.get("null_key"), Some(serde_json::Value::Null));
    // Boolean
    store.set("bool_key", serde_json::json!(true));
    assert_eq!(store.get("bool_key"), Some(serde_json::json!(true)));
    // Array
    store.set("array_key", serde_json::json!([1, 2, 3]));
    assert_eq!(store.get("array_key"), Some(serde_json::json!([1, 2, 3])));
    // Object
    store.set("obj_key", serde_json::json!({"nested": "value"}));
    assert_eq!(
      store.get("obj_key"),
      Some(serde_json::json!({"nested": "value"}))
    );
    // Empty string key
    store.set("", serde_json::json!("empty_key_value"));
    assert_eq!(store.get(""), Some(serde_json::json!("empty_key_value")));
  }

  #[test]
  fn test_signal_store_update_with_function() {
    let store = SignalStore::new();
    store.set("text", serde_json::json!("hello"));
    store.update("text", |v| {
      serde_json::json!(format!("{} world", v.as_str().unwrap()))
    });
    assert_eq!(store.get("text"), Some(serde_json::json!("hello world")));
  }

  #[test]
  fn test_signal_store_delete_nonexistent() {
    let store = SignalStore::new();
    // Deleting a non-existent key should not panic
    store.delete("nonexistent");
    assert_eq!(store.get("nonexistent"), None);
  }

  #[test]
  fn test_signal_store_subscription_notifications() {
    use std::sync::{Arc, Mutex};

    let store = SignalStore::new();
    let notifications: Arc<Mutex<Vec<(String, serde_json::Value)>>> =
      Arc::new(Mutex::new(Vec::new()));
    let notifications_clone = notifications.clone();

    store.subscribe(move |key, value| {
      if let Ok(mut notifications) = notifications_clone.lock() {
        notifications.push((key.to_string(), value.clone()));
      }
    });
    store.set("key1", serde_json::json!("value1"));
    store.set("key2", serde_json::json!("value2"));
    store.update("key1", |_v| serde_json::json!("updated"));

    let notifications = notifications.lock().unwrap();
    assert_eq!(notifications.len(), 3);
    assert_eq!(notifications[0].0, "key1");
    assert_eq!(notifications[0].1, serde_json::json!("value1"));
    assert_eq!(notifications[1].0, "key2");
    assert_eq!(notifications[2].0, "key1");
    assert_eq!(notifications[2].1, serde_json::json!("updated"));
  }

  #[test]
  fn test_signal_store_keys() {
    let store = SignalStore::new();
    store.set("key1", serde_json::json!("value1"));
    store.set("key2", serde_json::json!("value2"));
    let keys = store.keys();
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
  }

  #[test]
  fn test_signal_store_to_json_from_json() {
    let store = SignalStore::new();
    store.set("key1", serde_json::json!("value1"));
    store.set("key2", serde_json::json!(42));
    let json = store.to_json();
    assert_eq!(json["key1"], serde_json::json!("value1"));
    assert_eq!(json["key2"], serde_json::json!(42));
    let store2 = SignalStore::new();
    store2.from_json(json.clone());
    assert_eq!(store2.get("key1"), Some(serde_json::json!("value1")));
    assert_eq!(store2.get("key2"), Some(serde_json::json!(42)));
  }

  #[test]
  fn test_signal_store_update_nonexistent_key() {
    let store = SignalStore::new();
    // Update on non-existent key should not panic
    store.update("nonexistent", |_v| serde_json::json!("updated"));
    assert_eq!(store.get("nonexistent"), None);
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
