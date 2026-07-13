use axum::{extract::Path, routing::{get, post, put, delete}, Router, Json};
use crate::schema::*;
use crate::http_client::HttpResponse;

pub fn create_schema_router() -> Router {
    Router::new()
        .route("/api/schemas", get(list_schemas))
        .route("/api/schema/create", post(create_schema))
        .route("/api/schema/:id", get(get_schema))
        .route("/api/schema/:id", put(update_schema))
        .route("/api/schema/:id", delete(delete_schema))
}

pub async fn list_schemas() -> Json<HttpResponse<Vec<SchemaDTO>>> {
    let schemas = db_get_all_schemas().await.unwrap_or_default();
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schemas retrieved successfully".to_string()),
        data: Some(schemas),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn create_schema(
    Json(payload): Json<CreateSchemaRequest>,
) -> Json<HttpResponse<SchemaDTO>> {
    let schema = SchemaDTO::new(payload.id.as_str());
    if let Err(e) = db_save_schema(&payload.id, schema).await {
        eprintln!("create_schema error: {}", e);
    }
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema created successfully".to_string()),
        data: Some(schema),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn get_schema(
    Path(id): Path<String>,
) -> Json<HttpResponse<SchemaDTO>> {
    let schema = db_get_schema(&id).await.unwrap_or_else(|e| {
        eprintln!("get_schema error: {}", e);
        None
    });
    Json(HttpResponse {
        status: "success".to_string(),
        message: Some("Schema retrieved successfully".to_string()),
        data: Some(schema),
        timestamp: chrono::Utc::now().timestamp(),
    })
}

pub async fn update_schema(
    Path(id): Path<String>,
    Json(payload): Json<SchemaDTO>,
) -> Json<HttpResponse<SchemaDTO>> {
    if let Err(e) = db_save_schema(&id, payload).await {
        eprintln!("update_schema error: {}", e);
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
    if let Err(e) = db_delete_schema(&id).await {
        eprintln!("delete_schema error: {}", e);
    }
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
