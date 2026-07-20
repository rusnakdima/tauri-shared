//! CRUD macro for generating Tauri commands from JsonProvider.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nosql_orm::provider::DatabaseProvider;
//! use nosql_orm::providers::JsonProvider;
//! use tauri::State;
//! use tauri_shared::define_json_crud_routes;
//!
//! // JsonProvider is managed via `app.manage(provider)` in lib.rs
//! define_json_crud_routes!(
//!     prefix: test_entity,
//!     table: "test_entities"
//! );
//! ```
//!
//! This generates 5 commands:
//! - `{prefix}_get`      → `DatabaseProvider::find_by_id(table, &id)`
//! - `{prefix}_get_all`  → `DatabaseProvider::find_all(table)`
//! - `{prefix}_create`   → `DatabaseProvider::insert(table, data)` with auto-generated UUID
//! - `{prefix}_update`   → `DatabaseProvider::update(table, &id, data)`
//! - `{prefix}_delete`   → `DatabaseProvider::delete(table, &id)`
//!
//! All commands use `#[tauri::command(rename_all = "snake_case")]` and return
//! `Result<Response<T>, String>` with proper Response status codes.

#[macro_export]
macro_rules! define_json_crud_routes {
    (
        prefix: $prefix:ident,
        table: $table:expr
    ) => {
        paste::paste! {
            /// Get a single entity by ID
            #[allow(dead_code)]
            #[tauri::command(rename_all = "snake_case")]
            pub async fn [<$prefix _get>](
                id: String,
                db: tauri::State<'_, nosql_orm::providers::JsonProvider>,
            ) -> Result<tauri_shared::response::Response<serde_json::Value>, String> {
                match db.find_by_id($table, &id).await {
                    Ok(Some(data)) => Ok(tauri_shared::response::Response::success(data, Some("Found"))),
                    Ok(None) => Ok(tauri_shared::response::Response::not_found(stringify!($prefix))),
                    Err(e) => Ok(tauri_shared::response::Response::error(e.to_string())),
                }
            }

            /// Get all entities
            #[allow(dead_code)]
            #[tauri::command(rename_all = "snake_case")]
            pub async fn [<$prefix _get_all>](
                db: tauri::State<'_, nosql_orm::providers::JsonProvider>,
            ) -> Result<tauri_shared::response::Response<Vec<serde_json::Value>>, String> {
                match db.find_all($table).await {
                    Ok(items) => Ok(tauri_shared::response::Response::success(items, Some("Found"))),
                    Err(e) => Ok(tauri_shared::response::Response::error(e.to_string())),
                }
            }

            /// Create a new entity with auto-generated UUID
            #[allow(dead_code)]
            #[tauri::command(rename_all = "snake_case")]
            pub async fn [<$prefix _create>](
                data: serde_json::Value,
                db: tauri::State<'_, nosql_orm::providers::JsonProvider>,
            ) -> Result<tauri_shared::response::Response<serde_json::Value>, String> {
                let id = format!("{}_{}", stringify!($prefix), uuid::Uuid::new_v4());
                let mut item = data;
                if let Some(obj) = item.as_object_mut() {
                    obj.insert("id".to_string(), serde_json::Value::String(id.clone()));
                }
                match db.insert($table, item).await {
                    Ok(_) => Ok(tauri_shared::response::Response::created(serde_json::json!({"id": id}))),
                    Err(e) => Ok(tauri_shared::response::Response::error(e.to_string())),
                }
            }

            /// Full update using DatabaseProvider::update
            #[allow(dead_code)]
            #[tauri::command(rename_all = "snake_case")]
            pub async fn [<$prefix _update>](
                id: String,
                data: serde_json::Value,
                db: tauri::State<'_, nosql_orm::providers::JsonProvider>,
            ) -> Result<tauri_shared::response::Response<serde_json::Value>, String> {
                let mut item = data;
                if let Some(obj) = item.as_object_mut() {
                    obj.insert("id".to_string(), serde_json::Value::String(id.clone()));
                }
                match db.update($table, &id, item).await {
                    Ok(_) => Ok(tauri_shared::response::Response::updated(serde_json::json!({"id": id}))),
                    Err(e) => Ok(tauri_shared::response::Response::error(e.to_string())),
                }
            }

            /// Delete an entity
            #[allow(dead_code)]
            #[tauri::command(rename_all = "snake_case")]
            pub async fn [<$prefix _delete>](
                id: String,
                db: tauri::State<'_, nosql_orm::providers::JsonProvider>,
            ) -> Result<tauri_shared::response::Response<serde_json::Value>, String> {
                match db.delete($table, &id).await {
                    Ok(true) => Ok(tauri_shared::response::Response::deleted(serde_json::Value::Null)),
                    Ok(false) => Ok(tauri_shared::response::Response::error("Entity not found".to_string())),
                    Err(e) => Ok(tauri_shared::response::Response::error(e.to_string())),
                }
            }
        }
    };
}
