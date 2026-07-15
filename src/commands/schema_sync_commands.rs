//! Schema sync commands - sync schemas from MongoDB cloud to local JSON.

use crate::log_info;
use crate::response::Response;
use crate::storage::SchemaSyncState;
use std::sync::Arc;

/// Try local schema first, fall back to cloud sync if not found.
/// This is the main command apps should call to get schemas.
#[tauri::command]
pub async fn get_schema_local_first(
  id: String,
  state: tauri::State<'_, Arc<SchemaSyncState>>,
) -> Result<Response<serde_json::Value>, String> {
  let sync = &state.service;

  // 1. Try local JSON file first (fast, offline-capable)
  match sync.get_local_schema().await {
    Ok(Some(schema)) => {
      log_info!("Schema '{}' loaded from local JSON file", id);
      return Ok(Response {
        status: crate::response::Status::Success,
        message: "Schema from local cache".to_string(),
        data: Some(schema),
      });
    }
    Ok(None) => {
      log_info!("Schema '{}' not found locally, will sync from cloud", id);
    }
    Err(e) => {
      log_info!("Error reading local schema '{}': {}, will try cloud", id, e);
    }
  }

  // 2. Not found locally, sync from MongoDB
  match sync.sync_schema(&id).await {
    Ok(schema) => {
      log_info!("Schema '{}' synced from MongoDB to local", id);
      Ok(Response {
        status: crate::response::Status::Success,
        message: "Schema synced from cloud".to_string(),
        data: Some(schema),
      })
    }
    Err(e) => {
      log_info!("Failed to sync schema '{}': {}", id, e);
      Ok(Response {
        status: crate::response::Status::NotFound,
        message: format!("Schema {} not found: {}", id, e),
        data: None,
      })
    }
  }
}

/// Force sync schema from MongoDB to local JSON file.
#[tauri::command]
pub async fn sync_schema_from_cloud(
  id: String,
  state: tauri::State<'_, Arc<SchemaSyncState>>,
) -> Result<Response<serde_json::Value>, String> {
  let sync = &state.service;

  match sync.sync_schema(&id).await {
    Ok(schema) => {
      log_info!("Schema '{}' force-synced from MongoDB", id);
      Ok(Response {
        status: crate::response::Status::Success,
        message: "Schema synced from cloud".to_string(),
        data: Some(schema),
      })
    }
    Err(e) => {
      log_info!("Failed to sync schema '{}': {}", id, e);
      Ok(Response {
        status: crate::response::Status::NotFound,
        message: format!("Sync failed: {}", e),
        data: None,
      })
    }
  }
}
