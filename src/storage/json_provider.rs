//! JSON Provider wrapper for nosql_orm JsonProvider.
//!
//! This module provides a Tauri-compatible wrapper around nosql_orm's JsonProvider,
//! allowing it to be used as application state in Tauri commands.

use nosql_orm::prelude::*;
use nosql_orm::providers::json::JsonProviderConfig;
use std::path::Path;
use std::sync::Arc;

/// Type alias for the JSON provider state used in Tauri commands.
pub type JsonProviderState = Arc<JsonProvider>;

/// Create a new JsonProvider instance with the given data directory.
pub async fn create_json_provider(data_dir: impl AsRef<Path>) -> OrmResult<JsonProvider> {
  JsonProvider::new(data_dir).await
}

/// Create a new JsonProvider instance with custom configuration.
pub async fn create_json_provider_with_config(
  config: JsonProviderConfig,
) -> OrmResult<JsonProvider> {
  JsonProvider::with_config(config).await
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::tempdir;

  #[tokio::test]
  async fn test_create_provider() {
    let dir = tempdir().unwrap();
    let provider = create_json_provider(dir.path()).await;
    assert!(provider.is_ok());
  }

  #[tokio::test]
  async fn test_insert_and_find() {
    let dir = tempdir().unwrap();
    let provider = create_json_provider(dir.path()).await.unwrap();

    let doc = serde_json::json!({
        "_id": "test-1",
        "name": "Test Document",
        "value": 42
    });

    let inserted = provider.insert("test_collection", doc).await;
    assert!(inserted.is_ok());

    let found = provider.find_by_id("test_collection", "test-1").await;
    assert!(found.is_ok());
    assert!(found.unwrap().is_some());
  }
}
