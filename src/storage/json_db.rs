use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::RwLock;

pub struct JsonDb {
    path: std::path::PathBuf,
    data: RwLock<HashMap<String, Value>>,
}

impl JsonDb {
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Ok(Self {
            path,
            data: RwLock::new(data),
        })
    }

    pub fn insert(&self, collection: &str, id: &str, doc: Value) -> std::io::Result<()> {
        {
            let mut data = self.data.write().unwrap();
            let coll = data.entry(collection.to_string()).or_insert_with(|| Value::Object(serde_json::Map::new()));
            if let Value::Object(ref mut map) = coll {
                map.insert(id.to_string(), doc);
            }
        }
        self.save()
    }

    pub fn find(&self, collection: &str, id: &str) -> Option<Value> {
        let data = self.data.read().unwrap();
        data.get(collection)
            .and_then(|coll| coll.get(id))
            .cloned()
    }

    pub fn find_all(&self, collection: &str) -> Vec<Value> {
        let data = self.data.read().unwrap();
        data.get(collection)
            .and_then(|coll| coll.as_object())
            .map(|map| map.values().cloned().collect())
            .unwrap_or_default()
    }

    pub fn update(&self, collection: &str, id: &str, doc: Value) -> std::io::Result<()> {
        self.insert(collection, id, doc)
    }

    pub fn delete(&self, collection: &str, id: &str) -> std::io::Result<()> {
        {
            let mut data = self.data.write().unwrap();
            if let Some(coll) = data.get_mut(collection) {
                if let Value::Object(ref mut map) = coll {
                    map.remove(id);
                }
            }
        }
        self.save()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let data = self.data.read().unwrap();
        let json = serde_json::to_string_pretty(&*data)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, json)
    }
}