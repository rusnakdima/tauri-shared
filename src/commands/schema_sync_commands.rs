use crate::log_error;
use crate::log_info;
use crate::response::Response;
use crate::storage::json_provider::JsonProviderState;
use crate::storage::SchemaSyncState;
use nosql_orm::provider::DatabaseProvider;
use std::sync::Arc;

/// Get schema from local JsonProvider (always reads from local app data dir).
/// Frontend calls this to load the schema for the current app.
#[tauri::command]
pub async fn get_schema(
  id: String,
  db: tauri::State<'_, JsonProviderState>,
) -> Result<Response<serde_json::Value>, String> {
  log_info!("[BACKEND] CMD:get_schema START id={}", id);
  let start = std::time::Instant::now();

  let data = db
    .find_by_id("schemas", &id)
    .await
    .map_err(|e| format!("Failed to query schema: {}", e))?;

  match data {
    Some(schema) => {
      log_info!(
        "[BACKEND] CMD:get_schema OK ({:?}) - found in JsonProvider",
        start.elapsed()
      );
      Ok(Response::success(
        schema,
        Some("Schema loaded from local storage"),
      ))
    }
    None => {
      log_info!(
        "[BACKEND] CMD:get_schema OK ({:?}) - not found in JsonProvider",
        start.elapsed()
      );
      Ok(Response::not_found(format!("Schema '{}'", id)))
    }
  }
}

/// Force sync schema from MongoDB to local JsonProvider.
/// Call this if you need to pull the latest schema from the cloud.
#[tauri::command]
pub async fn sync_schema_from_cloud(
  id: String,
  sync_state: tauri::State<'_, Arc<SchemaSyncState>>,
  db: tauri::State<'_, JsonProviderState>,
) -> Result<Response<serde_json::Value>, String> {
  log_info!("[BACKEND] CMD:sync_schema_from_cloud START id={}", id);
  let start = std::time::Instant::now();

  match sync_state.service.sync_schema(&id, &db).await {
    Ok(schema) => {
      log_info!(
        "[BACKEND] CMD:sync_schema_from_cloud OK ({:?}) - synced from MongoDB",
        start.elapsed()
      );
      Ok(Response::success(schema, Some("Schema synced from cloud")))
    }
    Err(e) => {
      log_error!(
        "[BACKEND] CMD:sync_schema_from_cloud ERROR ({:?}): {}",
        start.elapsed(),
        e
      );
      Ok(Response::error(format!("Sync failed: {}", e)))
    }
  }
}
