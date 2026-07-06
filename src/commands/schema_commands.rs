use crate::{log_info, AppError, Response, UiSchema};
use nosql_orm::provider::DatabaseProvider;
use nosql_orm::providers::JsonProvider;

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
    db: tauri::State<'_, JsonProvider>,
    id: String,
) -> Result<Response<serde_json::Value>, AppError> {
    log_info!("get_ui_schema called with id: {}", id);
    let data = db.find_by_id("schemas", &id).await?;
    match data {
        Some(schema_data) => {
            log_info!(
                "Schema '{}' found, size: {} bytes",
                id,
                schema_data.to_string().len()
            );
            Ok(Response::success(schema_data, Some("Schema loaded")))
        }
        None => {
            log_info!("Schema '{}' not found in database", id);
            Err(AppError::NotFound(format!("Schema '{}' not found", id)))
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
