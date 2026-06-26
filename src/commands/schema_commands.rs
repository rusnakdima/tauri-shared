use crate::{AppError, UiSchema};
use nosql_orm::provider::DatabaseProvider;

#[tauri::command]
pub async fn get_schema(
    db: tauri::State<'_, impl DatabaseProvider>,
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
    db: tauri::State<'_, impl DatabaseProvider>,
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
    db: tauri::State<'_, impl DatabaseProvider>,
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
pub async fn delete_schema(
    db: tauri::State<'_, impl DatabaseProvider>,
    id: String,
) -> Result<(), AppError> {
    db.delete("schemas", &id).await.map_err(AppError::from)?;
    Ok(())
}
