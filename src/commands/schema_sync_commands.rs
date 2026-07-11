use crate::response::{Response, Status};
use std::sync::Mutex;

pub struct SchemaSyncState {
    pub service: Mutex<Option<()>>,
}

impl Default for SchemaSyncState {
    fn default() -> Self {
        Self {
            service: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn get_schema_local_first(
    _app_id: String,
    _state: tauri::State<'_, SchemaSyncState>,
) -> Result<Response<serde_json::Value>, String> {
    Ok(Response {
        status: Status::NotFound,
        message: "Local schema sync not yet implemented".to_string(),
        data: None,
    })
}

#[tauri::command]
pub async fn save_schema_local(
    _app_id: String,
    _schema: serde_json::Value,
    _state: tauri::State<'_, SchemaSyncState>,
) -> Result<Response<()>, String> {
    Ok(Response {
        status: Status::NotFound,
        message: "Local schema sync not yet implemented".to_string(),
        data: None,
    })
}

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
