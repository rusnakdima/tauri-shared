use crate::log_info;
use reqwest::Client;
use serde_json::Value;

// Designer MongoDB connection constants for schema sync
// Note: This is the REST API endpoint, not the native MongoDB port
const DESIGNER_MONGODB_URI: &str = "http://localhost:3000";
const DESIGNER_DATABASE: &str = "designer_db";
const SCHEMA_COLLECTION: &str = "schemas";

pub struct MongoBridge {
  endpoint: String,
  client: Client,
}

impl MongoBridge {
  pub fn new(endpoint: &str) -> Self {
    Self {
      endpoint: endpoint.to_string(),
      client: Client::new(),
    }
  }

  /// Create a MongoBridge connected to the Designer database
  pub fn designer() -> Self {
    Self::new(DESIGNER_MONGODB_URI)
  }

  /// Get the Designer database name
  pub fn designer_database() -> &'static str {
    DESIGNER_DATABASE
  }

  /// Get the schema collection name
  pub fn schema_collection() -> &'static str {
    SCHEMA_COLLECTION
  }

  pub async fn insert(&self, collection: &str, id: &str, doc: Value) -> Result<(), String> {
    let url = format!("{}/{}/{}", self.endpoint, collection, id);
    self
      .client
      .post(&url)
      .json(&doc)
      .send()
      .await
      .map_err(|e| e.to_string())?;
    log_info!("MongoDB insert: {}/{} -> {:?}", collection, id, doc);
    Ok(())
  }

  pub async fn update(&self, collection: &str, id: &str, doc: Value) -> Result<(), String> {
    let url = format!("{}/{}/{}", self.endpoint, collection, id);
    self
      .client
      .put(&url)
      .json(&doc)
      .send()
      .await
      .map_err(|e| e.to_string())?;
    log_info!("MongoDB update: {}/{} -> {:?}", collection, id, doc);
    Ok(())
  }

  pub async fn delete(&self, collection: &str, id: &str) -> Result<(), String> {
    let url = format!("{}/{}/{}", self.endpoint, collection, id);
    self
      .client
      .delete(&url)
      .send()
      .await
      .map_err(|e| e.to_string())?;
    log_info!("MongoDB delete: {}/{}", collection, id);
    Ok(())
  }

  /// Fetch a schema by ID from the Designer database
  pub async fn fetch_schema(&self, schema_id: &str) -> Result<Value, String> {
    let url = format!("{}/{}/{}", self.endpoint, SCHEMA_COLLECTION, schema_id);
    let response = self
      .client
      .get(&url)
      .send()
      .await
      .map_err(|e| e.to_string())?
      .json::<Value>()
      .await
      .map_err(|e| e.to_string())?;
    log_info!("Fetched schema {} from Designer", schema_id);
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use wiremock::matchers::{method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  #[tokio::test]
  async fn test_insert_posts_to_collection_endpoint() {
    let mock_server = MockServer::start().await;
    let bridge = MongoBridge::new(&mock_server.uri());

    Mock::given(method("POST"))
      .and(path("/test_collection"))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let doc = serde_json::json!({"name": "test", "value": 42});
    let result = bridge.insert("test_collection", "doc1", doc).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_update_puts_to_collection_id_endpoint() {
    let mock_server = MockServer::start().await;
    let bridge = MongoBridge::new(&mock_server.uri());

    Mock::given(method("PUT"))
      .and(path("/test_collection/doc1"))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let doc = serde_json::json!({"name": "updated"});
    let result = bridge.update("test_collection", "doc1", doc).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_delete_deletes_from_collection_id_endpoint() {
    let mock_server = MockServer::start().await;
    let bridge = MongoBridge::new(&mock_server.uri());

    Mock::given(method("DELETE"))
      .and(path("/test_collection/doc1"))
      .respond_with(ResponseTemplate::new(200))
      .mount(&mock_server)
      .await;

    let result = bridge.delete("test_collection", "doc1").await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_insert_returns_err_on_network_failure() {
    let bridge = MongoBridge::new("http://invalid-host-that-does-not-exist:99999");
    let doc = serde_json::json!({"name": "test"});
    let result = bridge.insert("collection", "id", doc).await;
    assert!(result.is_err());
  }
}
