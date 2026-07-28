use crate::crud::service::CrudService;
use crate::response::Response;
use nosql_orm::prelude::*;
use serde::Deserialize;
use serde_json::Value;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrudRequest {
  pub table: String,
  pub operation: String,
  pub id: Option<String>,
  pub data: Option<Value>,
  pub filter: Option<Value>,
  pub skip: Option<usize>,
  pub limit: Option<usize>,
  pub sort_by: Option<String>,
  pub sort_asc: Option<bool>,
}

#[tauri::command]
pub async fn crud_execute(
  request: CrudRequest,
  db: tauri::State<'_, JsonProvider>,
) -> Result<Response<Value>, String> {
  let service = CrudService::new(&*db);
  service
    .execute(
      &request.operation,
      &request.table,
      request.id.as_deref(),
      request.data,
      request.filter,
    )
    .await
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::crud::service::CrudService;
  use crate::response::Status;
  use tempfile::TempDir;

  fn entity_path(temp_dir: &TempDir, table: &str) -> std::path::PathBuf {
    temp_dir.path().join(format!("{}.json", table))
  }

  fn write_entity(temp_dir: &TempDir, table: &str, data: &str) {
    std::fs::write(entity_path(temp_dir, table), data).unwrap();
  }

  async fn make_service(temp_dir: &TempDir) -> CrudService {
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    CrudService::new(Arc::new(provider))
  }

  #[tokio::test]
  async fn test_crud_request_serialization() {
    let json = r#"{"table":"users","operation":"get","id":"123","data":null,"filter":null,"skip":0,"limit":10,"sortBy":"name","sortAsc":true}"#;
    let req: CrudRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.table, "users");
    assert_eq!(req.operation, "get");
    assert_eq!(req.id, Some("123".to_string()));
    assert_eq!(req.skip, Some(0));
    assert_eq!(req.limit, Some(10));
    assert_eq!(req.sort_by, Some("name".to_string()));
    assert_eq!(req.sort_asc, Some(true));
  }

  #[tokio::test]
  async fn test_crud_execute_create() {
    let temp_dir = TempDir::new().unwrap();
    let service = make_service(&temp_dir).await;

    let data = serde_json::json!({"name": "Alice", "email": "alice@example.com"});
    let result = service
      .execute("create", "users", None, Some(data), None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Created);
    let created = result.data.unwrap();
    assert_eq!(created.get("name").unwrap(), "Alice");
    assert!(
      created.get("id").is_some(),
      "created entity must have an id"
    );
  }

  #[tokio::test]
  async fn test_crud_execute_get() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "users",
      r#"[{"id":"u1","name":"Bob","email":"bob@example.com"}]"#,
    );
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("get", "users", Some("u1"), None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Success);
    let data = result.data.unwrap();
    assert_eq!(data.get("name").unwrap(), "Bob");
  }

  #[tokio::test]
  async fn test_crud_execute_get_all() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "products",
      r#"[
        {"id":"p1","name":"Apple"},
        {"id":"p2","name":"Banana"}
      ]"#,
    );
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("get_all", "products", None, None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Success);
    let data = result.data.unwrap();
    let arr = data.as_array().unwrap();
    assert_eq!(arr.len(), 2);
  }

  #[tokio::test]
  async fn test_crud_execute_update() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "users",
      r#"[{"id":"u1","name":"Charlie","email":"charlie@example.com"}]"#,
    );
    let service = make_service(&temp_dir).await;

    let data = serde_json::json!({"name": "Charles", "email": "charles@example.com"});
    let result = service
      .execute("update", "users", Some("u1"), Some(data), None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Updated);
    let updated = result.data.unwrap();
    assert_eq!(updated.get("name").unwrap(), "Charles");
  }

  #[tokio::test]
  async fn test_crud_execute_patch() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "users",
      r#"[{"id":"u1","name":"Dave","email":"dave@example.com","age":30}]"#,
    );
    let service = make_service(&temp_dir).await;

    let patch = serde_json::json!({"age": 31});
    let result = service
      .execute("patch", "users", Some("u1"), Some(patch), None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Updated);
    let patched = result.data.unwrap();
    assert_eq!(patched.get("age").unwrap(), 31);
    assert_eq!(patched.get("name").unwrap(), "Dave");
  }

  #[tokio::test]
  async fn test_crud_execute_delete() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "users",
      r#"[{"id":"u1","name":"Eve"},{"id":"u2","name":"Frank"}]"#,
    );
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("delete", "users", Some("u1"), None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Deleted);

    let remaining = service
      .execute("get_all", "users", None, None, None)
      .await
      .unwrap();
    let data = remaining.data.unwrap();
    let arr = data.as_array().unwrap();
    assert_eq!(arr.len(), 1);
  }

  #[tokio::test]
  async fn test_crud_execute_count() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(
      &temp_dir,
      "items",
      r#"[
        {"id":"1","name":"Item 1"},
        {"id":"2","name":"Item 2"},
        {"id":"3","name":"Item 3"}
      ]"#,
    );
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("count", "items", None, None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Success);
    let count = result.data.unwrap().as_i64().unwrap();
    assert_eq!(count, 3);
  }

  #[tokio::test]
  async fn test_crud_execute_exists() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(&temp_dir, "users", r#"[{"id":"u1","name":"Grace"}]"#);
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("exists", "users", Some("u1"), None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::Success);
    assert_eq!(result.data.unwrap().as_bool().unwrap(), true);
  }

  #[tokio::test]
  async fn test_crud_execute_get_not_found() {
    let temp_dir = TempDir::new().unwrap();
    write_entity(&temp_dir, "users", r#"[{"id":"u1","name":"Henry"}]"#);
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("get", "users", Some("nonexistent"), None, None)
      .await
      .unwrap();

    assert_eq!(result.status, Status::NotFound);
  }

  #[tokio::test]
  async fn test_crud_execute_unknown_operation() {
    let temp_dir = TempDir::new().unwrap();
    let service = make_service(&temp_dir).await;

    let result = service
      .execute("unknown_op", "users", None, None, None)
      .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Unknown operation"));
  }

  #[tokio::test]
  async fn test_crud_execute_get_requires_id() {
    let temp_dir = TempDir::new().unwrap();
    let service = make_service(&temp_dir).await;

    let result = service.execute("get", "users", None, None, None).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("ID required"));
  }
}
