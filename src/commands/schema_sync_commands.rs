use crate::response::{Response, Status};
use crate::sync::schema_sync::SchemaSyncService;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SchemaSyncState {
    pub service: Mutex<Option<SchemaSyncService>>,
}

#[tauri::command]
pub async fn get_schema_local_first(
    app_id: String,
    state: tauri::State<'_, Arc<SchemaSyncState>>,
) -> Result<Response<serde_json::Value>, String> {
    let guard = state.service.lock().await;
    if let Some(service) = guard.as_ref() {
        match service.get_schema_local(&app_id).await {
            Ok(Some(schema)) => {
                let json = serde_json::to_value(&schema)
                    .map_err(|e| format!("Failed to serialize: {}", e))?;
                Ok(Response::success(json, Some("Schema loaded from local cache")))
            }
            Ok(None) => Ok(Response::error("No schema found in local cache")),
            Err(e) => Ok(Response::error(&e)),
        }
    } else {
        Ok(Response::error("SchemaSyncService not initialized"))
    }
}

#[tauri::command]
pub async fn save_schema_local(
    app_id: String,
    schema: serde_json::Value,
    state: tauri::State<'_, Arc<SchemaSyncState>>,
) -> Result<Response<()>, String> {
    let guard = state.service.lock().await;
    if let Some(service) = guard.as_ref() {
        let schema: crate::schema::UiSchema = serde_json::from_value(schema)
            .map_err(|e| format!("Invalid schema: {}", e))?;
        service
            .save_schema_local(&app_id, &schema)
            .await
            .map_err(|e| format!("Failed to save: {}", e))?;
        Ok(Response::success((), Some("Schema saved to local cache")))
    } else {
        Ok(Response::error("SchemaSyncService not initialized"))
    }
}

// TODO: implement cloud sync via MongoBridge once designer endpoint contract is finalized.
#[tauri::command]
#[deprecated(note = "Cloud sync not yet implemented — returns NotFound. Use Designer to manage schemas.")]
pub async fn sync_schema_from_cloud(
    _app_id: String,
    _designer_endpoint: String,
) -> Result<Response<serde_json::Value>, String> {
    Ok(Response {
        status: Status::NotFound,
        message: "Cloud sync not yet implemented - use Designer to manage schemas".to_string(),
        data: None,
    })
}
