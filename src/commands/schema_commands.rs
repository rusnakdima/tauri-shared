use crate::{log_info, AppError, Response, UiSchema};
use nosql_orm::provider::DatabaseProvider;
use nosql_orm::providers::JsonProvider;
use tauri::Manager;

#[tauri::command]
pub async fn get_schema(
  db: tauri::State<'_, JsonProvider>,
  id: String,
) -> Result<UiSchema, AppError> {
  let data = db
    .find_by_id("schemas", &id)
    .await
    .map_err(AppError::from)?
    .ok_or_else(|| AppError::NotFound(format!("Schema {} not found", id)))?;

  let schema: UiSchema = serde_json::from_value(data)
    .map_err(|e| AppError::ValidationError(format!("Invalid schema format: {}", e)))?;
  Ok(schema)
}

#[tauri::command]
pub async fn save_schema(
  db: tauri::State<'_, JsonProvider>,
  schema: UiSchema,
) -> Result<(), AppError> {
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
  } else {
    db.insert("schemas", data).await.map_err(AppError::from)?;
  }
  Ok(())
}

#[tauri::command]
pub async fn get_all_schemas(
  db: tauri::State<'_, JsonProvider>,
) -> Result<Vec<UiSchema>, AppError> {
  let items = db.find_all("schemas").await.map_err(AppError::from)?;

  let schemas: Vec<UiSchema> = items
    .into_iter()
    .map(|data| {
      serde_json::from_value(data)
        .map_err(|e| AppError::ValidationError(format!("Invalid schema format: {}", e)))
    })
    .collect::<Result<Vec<_>, _>>()?;
  Ok(schemas)
}

#[tauri::command]
pub async fn get_ui_schema(
  app: tauri::AppHandle,
  db: tauri::State<'_, JsonProvider>,
  id: String,
) -> Result<Response<serde_json::Value>, AppError> {
  log_info!("get_ui_schema called with id: {}", id);

  // Try JsonProvider first
  let data = db.find_by_id("schemas", &id).await?;
  if let Some(schema_data) = data {
    log_info!(
      "Schema '{}' found in JsonProvider, size: {} bytes",
      id,
      schema_data.to_string().len()
    );
    return Ok(Response::success(schema_data, Some("Schema loaded")));
  }

  // Fall back to system schema file: ~/.local/share/com.tcs.{id}/schemas.json
  log_info!(
    "Schema '{}' not found in JsonProvider, trying system schema file",
    id
  );
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
                        log_info!("Schema '{}' found in system schema file (array)", id);
                        // Optionally seed into JsonProvider for next time
                        let _ = db.insert("schemas", item.clone()).await;
                        return Ok(Response::success(
                          item,
                          Some("Schema loaded from system file"),
                        ));
                      }
                    }
                    log_info!("Schema '{}' not found in system schema file", id);
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                  serde_json::Value::Object(map) => {
                    // Single schema object - check if id matches
                    let schema_id = map.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    if schema_id == id || id.is_empty() {
                      log_info!("Schema '{}' found in system schema file (object)", id);
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
                      "Schema '{}' not found in system schema file (id mismatch)",
                      id
                    );
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                  _ => {
                    log_info!("Schema file has unexpected format");
                    return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
                  }
                }
              }
              Err(e) => {
                log_info!("Failed to parse system schema file: {}", e);
                return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
              }
            }
          }
          Err(e) => {
            log_info!("Failed to read system schema file: {}", e);
            return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
          }
        }
      } else {
        log_info!("System schema file does not exist: {:?}", schema_path);
        return Err(AppError::NotFound(format!("Schema '{}' not found", id)));
      }
    }
    Err(e) => {
      log_info!("Failed to get app data dir: {}", e);
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
  log_info!("save_ui_schema called with id: {}", id);
  if db.find_by_id("schemas", &id).await?.is_some() {
    db.update("schemas", &id, schema).await?;
    log_info!("Schema '{}' updated", id);
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
    log_info!("Schema '{}' inserted", id);
  }
  Ok(Response::success((), Some("Schema saved")))
}

#[tauri::command]
pub async fn delete_schema(db: tauri::State<'_, JsonProvider>, id: String) -> Result<(), AppError> {
  db.delete("schemas", &id).await.map_err(AppError::from)?;
  Ok(())
}
