use crate::log_info;
use serde_json::Value;

pub struct MongoBridge {
    endpoint: String,
}

impl MongoBridge {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn insert(&self, collection: &str, id: &str, doc: Value) -> Result<(), String> {
        let url = format!("{}/{}", self.endpoint, collection);
        log_info!("MongoDB insert: {}/{} -> {:?}", collection, id, doc);
        Ok(())
    }

    pub fn update(&self, collection: &str, id: &str, doc: Value) -> Result<(), String> {
        let url = format!("{}/{}/{}", self.endpoint, collection, id);
        log_info!("MongoDB update: {}/{} -> {:?}", collection, id, doc);
        Ok(())
    }

    pub fn delete(&self, collection: &str, id: &str) -> Result<(), String> {
        let url = format!("{}/{}/{}", self.endpoint, collection, id);
        log_info!("MongoDB delete: {}/{}", collection, id);
        Ok(())
    }
}