use super::kernel_db::KernelDb;

pub async fn db_get_schema<T: KernelDb>(
    db: &T,
    id: &str,
) -> crate::Result<Option<serde_json::Value>> {
    db.find_by_id("schemas", id)
        .await
        .map_err(crate::AppError::from)
}

pub async fn db_save_schema<T: KernelDb>(
    db: &T,
    id: &str,
    data: serde_json::Value,
) -> crate::Result<serde_json::Value> {
    if db
        .find_by_id("schemas", id)
        .await
        .map_err(crate::AppError::from)?
        .is_some()
    {
        db.update("schemas", id, data)
            .await
            .map_err(crate::AppError::from)?;
    } else {
        db.insert("schemas", data)
            .await
            .map_err(crate::AppError::from)?;
    }
    Ok(serde_json::json!({ "id": id }))
}

pub async fn db_get_all_schemas<T: KernelDb>(db: &T) -> crate::Result<Vec<serde_json::Value>> {
    db.find_all("schemas").await.map_err(crate::AppError::from)
}

pub async fn db_delete_schema<T: KernelDb>(db: &T, id: &str) -> crate::Result<()> {
    db.delete("schemas", id)
        .await
        .map_err(crate::AppError::from)
}