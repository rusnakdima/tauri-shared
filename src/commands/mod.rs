pub mod algorithm_commands;
pub mod logger_commands;
pub mod schema_commands;
pub mod sdui_commands;

use nosql_orm::prelude::Entity;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;

pub trait KernelEntity: Entity + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T: Entity + Serialize + DeserializeOwned + Send + Sync + 'static> KernelEntity for T {}

pub trait KernelDb: Send + Sync {
    fn find_by_id(
        &self,
        collection: &str,
        id: &str,
    ) -> impl Future<
        Output = std::result::Result<Option<serde_json::Value>, nosql_orm::error::OrmError>,
    > + Send;
    fn find_all(
        &self,
        collection: &str,
    ) -> impl Future<Output = std::result::Result<Vec<serde_json::Value>, nosql_orm::error::OrmError>>
           + Send;
    fn insert(
        &self,
        collection: &str,
        data: serde_json::Value,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;
    fn update(
        &self,
        collection: &str,
        id: &str,
        data: serde_json::Value,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;
    fn delete(
        &self,
        collection: &str,
        id: &str,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;
}

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
