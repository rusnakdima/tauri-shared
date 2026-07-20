use crate::response::Response;
use nosql_orm::prelude::*;
use serde_json::Value;
use std::sync::Arc;

pub struct CrudService {
  provider: Arc<JsonProvider>,
}

impl CrudService {
  pub fn new(provider: JsonProvider) -> Self {
    Self {
      provider: Arc::new(provider),
    }
  }

  pub async fn execute(
    &self,
    operation: &str,
    entity: &str,
    id: Option<&str>,
    data: Option<Value>,
    _filter: Option<Value>,
  ) -> Result<Response<Value>, String> {
    match operation {
      "get" => {
        let id = id.ok_or("ID required for get")?;
        let result = self
          .provider
          .find_by_id(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        match result {
          Some(data) => Ok(Response::success(data, Some("Found"))),
          None => Ok(Response::not_found(entity)),
        }
      }
      "get_all" => {
        let results = self
          .provider
          .find_all(entity)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(Value::Array(results), Some("Found")))
      }
      "create" | "save" => {
        let data = data.ok_or("Data required for create")?;
        let result = self
          .provider
          .insert(entity, data)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::created(result))
      }
      "update" => {
        let id = id.ok_or("ID required for update")?;
        let mut data = data.ok_or("Data required for update")?;
        if let Some(obj) = data.as_object_mut() {
          obj.insert("id".to_string(), Value::String(id.to_string()));
        }
        let result = self
          .provider
          .update(entity, id, data)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::updated(result))
      }
      "patch" => {
        let id = id.ok_or("ID required for patch")?;
        let patch = data.ok_or("Patch data required")?;
        let result = self
          .provider
          .patch(entity, id, patch)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::updated(result))
      }
      "delete" => {
        let id = id.ok_or("ID required for delete")?;
        self
          .provider
          .delete(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::deleted(Value::Null))
      }
      "count" => {
        let count = self
          .provider
          .count(entity, None)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(
          Value::Number(count.into()),
          Some("Count"),
        ))
      }
      "exists" => {
        let id = id.ok_or("ID required for exists")?;
        let exists = self
          .provider
          .exists(entity, id)
          .await
          .map_err(|e| e.to_string())?;
        Ok(Response::success(
          Value::Bool(exists),
          Some(if exists { "Exists" } else { "Not found" }),
        ))
      }
      _ => Err(format!("Unknown operation: {}", operation)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;

  #[tokio::test]
  async fn test_crud_get_all() {
    let temp_dir = TempDir::new().unwrap();
    let items_path = temp_dir.path().join("items.json");
    std::fs::write(
      &items_path,
      r#"[{"id": "1", "name": "Test Item", "value": 42}]"#,
    )
    .unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let crud = CrudService::new(provider);
    let result = crud.execute("get_all", "items", None, None, None).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.data.as_ref().expect("data must exist").is_array());
    let data = resp.data.unwrap();
    let arr = data.as_array().unwrap();
    assert!(!arr.is_empty());
  }

  #[tokio::test]
  async fn test_crud_count() {
    let temp_dir = TempDir::new().unwrap();
    let items_path = temp_dir.path().join("items.json");
    std::fs::write(&items_path, r#"[{"id": "1", "name": "Test Item"}]"#).unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let crud = CrudService::new(provider);
    let result = crud.execute("count", "items", None, None, None).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_crud_unknown_operation() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap())
      .await
      .unwrap();
    let crud = CrudService::new(provider);
    let result = crud.execute("invalid_op", "items", None, None, None).await;
    assert!(result.is_err());
  }
}
