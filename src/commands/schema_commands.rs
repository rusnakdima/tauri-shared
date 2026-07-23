use crate::log_error;
use crate::log_info;
use crate::{AppError, Response, UiSchema};
use nosql_orm::provider::DatabaseProvider;
use nosql_orm::providers::JsonProvider;
use tauri::Manager;

#[tauri::command]
pub async fn get_schema_direct(
  db: tauri::State<'_, JsonProvider>,
  id: String,
) -> Result<UiSchema, AppError> {
  log_info!("[BACKEND] CMD:get_schema_direct START id={}", id);
  let start = std::time::Instant::now();
  let data = db
    .find_by_id("schemas", &id)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound(format!("Schema {} not found", id)))?;

  let schema: UiSchema = serde_json::from_value(data)
    .map_err(|e| AppError::ValidationError(format!("Invalid schema format: {}", e)))?;
  log_info!("[BACKEND] CMD:get_schema_direct OK ({:?})", start.elapsed());
  Ok(schema)
}

#[tauri::command]
pub async fn save_schema(
  db: tauri::State<'_, JsonProvider>,
  schema: UiSchema,
) -> Result<(), AppError> {
  log_info!("[BACKEND] CMD:save_schema START");
  let start = std::time::Instant::now();
  let id = schema.schema_version.clone();
  let data = serde_json::to_value(&schema)
    .map_err(|e| AppError::ValidationError(format!("Failed to serialize schema: {}", e)))?;

  if db
    .find_by_id("schemas", &id)
    .await
    .map_err(AppError::from)?
    .is_some()
  {
    db.update("schemas", &id, data)
      .await
      .map_err(AppError::from)?;
    log_info!(
      "[BACKEND] CMD:save_schema OK ({:?}) - updated",
      start.elapsed()
    );
  } else {
    db.insert("schemas", data).await.map_err(AppError::from)?;
    log_info!(
      "[BACKEND] CMD:save_schema OK ({:?}) - inserted",
      start.elapsed()
    );
  }
  Ok(())
}

#[tauri::command]
pub async fn get_all_schemas(
  db: tauri::State<'_, JsonProvider>,
) -> Result<Vec<UiSchema>, AppError> {
  log_info!("[BACKEND] CMD:get_all_schemas START");
  let start = std::time::Instant::now();
  let items = db.find_all("schemas").await.map_err(AppError::from)?;

  let schemas: Vec<UiSchema> = items
    .into_iter()
    .map(|data| {
      serde_json::from_value(data)
        .map_err(|e| AppError::ValidationError(format!("Invalid schema format: {}", e)))
    })
    .collect::<Result<Vec<_>, _>>()?;
  log_info!(
    "[BACKEND] CMD:get_all_schemas OK ({:?}) count={}",
    start.elapsed(),
    schemas.len()
  );
  Ok(schemas)
}

#[tauri::command]
pub async fn get_ui_schema(
  app: tauri::AppHandle,
  db: tauri::State<'_, JsonProvider>,
  id: String,
) -> Result<Response<serde_json::Value>, AppError> {
  log_info!("[BACKEND] CMD:get_ui_schema START id={}", id);
  let start = std::time::Instant::now();

  // Try JsonProvider first
  let data = db.find_by_id("schemas", &id).await?;
  if let Some(schema_data) = data {
    log_info!(
      "[BACKEND] CMD:get_ui_schema OK ({:?}) - found in JsonProvider ({} bytes)",
      start.elapsed(),
      schema_data.to_string().len()
    );
    return Ok(Response::success(schema_data, Some("Schema loaded")));
  }

  // Fall back to system schema file: ~/.local/share/com.tcs.{id}/schemas.json
  log_info!("[BACKEND] CMD:get_ui_schema - not in JsonProvider, trying system schema file",);
  match app.path().app_data_dir() {
    Ok(app_data_dir) => {
      let schema_path = app_data_dir.join("schemas.json");
      if schema_path.exists() {
        match std::fs::read_to_string(&schema_path) {
          Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
              Ok(schema_value) => {
                // Handle array format - find by id
                match schema_value {
                  serde_json::Value::Array(arr) => {
                    for item in arr {
                      if item.get("id").and_then(|v| v.as_str()) == Some(&id) {
                        log_info!(
                          "[BACKEND] CMD:get_ui_schema OK ({:?}) - found in system schema (array)",
                          start.elapsed()
                        );
                        // Optionally seed into JsonProvider for next time
                        let _ = db.insert("schemas", item.clone()).await;
                        return Ok(Response::success(
                          item,
                          Some("Schema loaded from system file"),
                        ));
                      }
                    }
                    log_info!(
                      "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - not found in system schema",
                      start.elapsed()
                    );
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                  serde_json::Value::Object(map) => {
                    // Single schema object - check if id matches
                    let schema_id = map.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    if schema_id == id || id.is_empty() {
                      log_info!(
                        "[BACKEND] CMD:get_ui_schema OK ({:?}) - found in system schema (object)",
                        start.elapsed()
                      );
                      // Optionally seed into JsonProvider for next time
                      let _ = db
                        .insert("schemas", serde_json::Value::Object(map.clone()))
                        .await;
                      return Ok(Response::success(
                        serde_json::Value::Object(map),
                        Some("Schema loaded from system file"),
                      ));
                    }
                    log_info!(
                      "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - id mismatch in system schema",
                      start.elapsed()
                    );
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                  _ => {
                    log_info!(
                      "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - unexpected schema format",
                      start.elapsed()
                    );
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                }
              }
              Err(e) => {
                log_info!(
                  "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - parse failed: {}",
                  start.elapsed(),
                  e
                );
                return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
              }
            }
          }
          Err(e) => {
            log_info!(
              "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - read failed: {}",
              start.elapsed(),
              e
            );
            return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
          }
        }
      } else {
        log_info!(
          "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - system schema file not found",
          start.elapsed()
        );
        return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
      }
    }
    Err(e) => {
      log_info!(
        "[BACKEND] CMD:get_ui_schema ERROR ({:?}) - app data dir error: {}",
        start.elapsed(),
        e
      );
      return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
    }
  }
}

#[tauri::command]
pub async fn save_ui_schema(
  db: tauri::State<'_, JsonProvider>,
  id: String,
  schema: serde_json::Value,
) -> Result<Response<()>, AppError> {
  log_info!("[BACKEND] CMD:save_ui_schema START id={}", id);
  let start = std::time::Instant::now();
  if db.find_by_id("schemas", &id).await?.is_some() {
    db.update("schemas", &id, schema).await?;
    log_info!(
      "[BACKEND] CMD:save_ui_schema OK ({:?}) - updated",
      start.elapsed()
    );
  } else {
    let mut doc = schema;
    if doc
      .get("id")
      .and_then(|v| v.as_str())
      .is_none_or(|s| s.is_empty())
    {
      doc["id"] = serde_json::json!(id);
    }
    db.insert("schemas", doc).await?;
    log_info!(
      "[BACKEND] CMD:save_ui_schema OK ({:?}) - inserted",
      start.elapsed()
    );
  }
  Ok(Response::success((), Some("Schema saved")))
}

#[tauri::command]
pub async fn delete_schema(db: tauri::State<'_, JsonProvider>, id: String) -> Result<(), AppError> {
  log_info!("[BACKEND] CMD:delete_schema START id={}", id);
  let start = std::time::Instant::now();
  db.delete("schemas", &id).await.map_err(AppError::from)?;
  log_info!("[BACKEND] CMD:delete_schema OK ({:?})", start.elapsed());
  Ok(())
}
