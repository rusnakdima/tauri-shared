use std::sync::Arc;

use crate::rbac::Permission;
use crate::runtime::SduiEngine;
use crate::schema::{DataBinding, RenderedPage, UiSchema};
use crate::sync::SyncEngine;
use crate::AppError;
use crate::Result;

pub type SduiEngineState = Arc<std::sync::RwLock<Option<SduiEngine>>>;

#[tauri::command]
pub async fn load_schema(
    schema: UiSchema,
    engine_state: tauri::State<'_, SduiEngineState>,
) -> Result<UiSchema> {
    let mut engine = SduiEngine::from(schema.clone());
    engine.load_schema(schema.clone());

    let mut guard = engine_state
        .write()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    *guard = Some(engine);

    Ok(schema)
}

#[tauri::command]
pub fn render_page(
    route: String,
    engine_state: tauri::State<'_, SduiEngineState>,
) -> Result<RenderedPage> {
    let guard = engine_state
        .read()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let engine = guard
        .as_ref()
        .ok_or_else(|| AppError::Internal("Engine not initialized".to_string()))?;
    engine.render_page(&route)
}

#[tauri::command]
pub fn resolve_binding(
    binding: DataBinding,
    engine_state: tauri::State<'_, SduiEngineState>,
) -> Result<serde_json::Value> {
    let guard = engine_state
        .read()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let engine = guard
        .as_ref()
        .ok_or_else(|| AppError::Internal("Engine not initialized".to_string()))?;
    engine.resolve_binding(&binding)
}

#[tauri::command]
pub async fn sync_to_cloud(engine: tauri::State<'_, Arc<SyncEngine>>) -> Result<()> {
    engine
        .sync_to_cloud()
        .await
        .map_err(|e| AppError::Internal(e))
}

#[tauri::command]
pub fn check_permission(permission: Permission, resource: String, action: String) -> Result<bool> {
    Ok(permission.matches(&resource, &action))
}
