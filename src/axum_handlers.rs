use axum::{extract::Path, routing::{get, post, put, delete}, Router, Json};
use serde::Deserialize;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::http_client::HttpResponse;

static SCHEMAS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn create_schema_router() -> Router {
    Router::new()
        .route("/api/schemas", get(list_schemas))
        .route("/api/schema/create", post(create_schema))
        .route("/api/schema/:id", get(get_schema))
        .route("/api/schema/:id", put(update_schema))
        .route("/api/schema/:id", delete(delete_schema))
}

pub async fn list_schemas() -> Json<HttpResponse<Vec<serde_json::Value>>> {
    let schemas = SCHEMAS.lock().unwrap().clone();
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schemas retrieved successfully".to_string()),
        data: Some(schemas),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn create_schema(
    Json(payload): Json<CreateSchemaRequest>,
) -> Json<HttpResponse<serde_json::Value>> {
    let schema = serde_json::json!({ "schema_version": payload.id });
    SCHEMAS.lock().unwrap().push(schema.clone());
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema created successfully".to_string()),
        data: Some(schema),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn get_schema(
    Path(id): Path<String>,
) -> Json<HttpResponse<serde_json::Value>> {
    let schemas = SCHEMAS.lock().unwrap();
    let schema = schemas.iter().find(|s| s.get("schema_version").and_then(|v| v.as_str()) == Some(&id)).cloned();
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema retrieved successfully".to_string()),
        data: schema,
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn update_schema(
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Json<HttpResponse<serde_json::Value>> {
    let mut schemas = SCHEMAS.lock().unwrap();
    if let Some(existing) = schemas.iter_mut().find(|s| s.get("schema_version").and_then(|v| v.as_str()) == Some(&id)) {
        *existing = payload.clone();
    }
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema updated successfully".to_string()),
        data: Some(payload),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn delete_schema(
    Path(id): Path<String>,
) -> Json<HttpResponse<()>> {
    let mut schemas = SCHEMAS.lock().unwrap();
    schemas.retain(|s| s.get("schema_version").and_then(|v| v.as_str()) != Some(&id));
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema deleted successfully".to_string()),
        data: Some(()),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

#[derive(Deserialize)]
pub struct CreateSchemaRequest {
    pub id: String,
}
