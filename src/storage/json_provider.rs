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

  fn create_temp_dir() -> std::path::PathBuf {
    let temp = std::env::temp_dir();
    let unique_dir = temp.join(format!("test_provider_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&unique_dir).unwrap();
    unique_dir
  }

  fn cleanup_temp_dir(path: &std::path::Path) {
    let _ = std::fs::remove_dir_all(path);
  }

  #[tokio::test]
  async fn test_create_provider() {
    let dir = create_temp_dir();
    let provider = create_json_provider(&dir).await;
    assert!(provider.is_ok());
    cleanup_temp_dir(&dir);
  }

  #[tokio::test]
  async fn test_find_nonexistent() {
    let dir = create_temp_dir();
    let provider = create_json_provider(&dir).await.unwrap();
    let found = provider
      .find_by_id("test_collection", "nonexistent-id")
      .await;
    assert!(found.is_ok());
    assert!(found.unwrap().is_none());
    cleanup_temp_dir(&dir);
  }

  #[tokio::test]
  async fn test_delete_existing() {
    let dir = create_temp_dir();
    let provider = create_json_provider(&dir).await.unwrap();

    let doc = serde_json::json!({
        "_id": "delete-test",
        "name": "To Be Deleted"
    });

    provider.insert("test_collection", doc).await.unwrap();
    let deleted = provider.delete("test_collection", "delete-test").await;
    assert!(deleted.is_ok());

    let found = provider
      .find_by_id("test_collection", "delete-test")
      .await
      .unwrap();
    assert!(found.is_none());
    cleanup_temp_dir(&dir);
  }

  #[tokio::test]
  async fn test_delete_nonexistent() {
    let dir = create_temp_dir();
    let provider = create_json_provider(&dir).await.unwrap();
    // Deleting non-existent should not panic
    let result = provider.delete("test_collection", "nonexistent-id").await;
    assert!(result.is_ok());
    cleanup_temp_dir(&dir);
  }
}
