use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::storage::signal_store::SignalStore;
use crate::AppError;
use crate::Result;

use super::engine::Callable;

#[derive(Clone)]
pub struct BindingResolver {
  store: Arc<SignalStore>,
  functions: HashMap<String, Callable>,
}

impl BindingResolver {
  pub fn new(store: Arc<SignalStore>, functions: HashMap<String, Callable>) -> Self {
    Self { store, functions }
  }

  pub fn resolve(&self, template: &str, context: &serde_json::Value) -> Result<String> {
    let mut result = template.to_string();
    let mut start = 0;

    while let Some(open_pos) = result[start..].find("{{") {
      let open_idx = start + open_pos;
      if let Some(close_pos) = result[open_idx + 2..].find("}}") {
        let close_idx = open_idx + 2 + close_pos;
        let path = result[open_idx + 2..close_idx].trim();

        if path.starts_with("data.") {
          let data_path = &path[5..];
          if let Some(value) = self.resolve_path(data_path, context) {
            result = result[..open_idx].to_string() + &value.to_string() + &result[close_idx + 2..];
            start = open_idx + value.to_string().len();
          } else {
            start = close_idx + 2;
          }
        } else if path.starts_with("env.") {
          let env_var = &path[4..];
          if let Ok(val) = std::env::var(env_var) {
            result = result[..open_idx].to_string() + &val + &result[close_idx + 2..];
            start = open_idx + val.len();
          } else {
            start = close_idx + 2;
          }
        } else {
          start = close_idx + 2;
        }
      } else {
        break;
      }
    }

    Ok(result)
  }

  fn resolve_path(&self, path: &str, data: &serde_json::Value) -> Option<serde_json::Value> {
    let parts: Vec<_> = path.split('.').collect();
    let mut current = data.clone();
    for part in parts {
      current = current.get(part)?.clone();
    }
    Some(current)
  }

  pub fn resolve_value(&self, value: &Value) -> Result<Value> {
    Ok(value.clone())
  }

  pub fn bind_function(&mut self, name: &str, func: Callable) {
    self.functions.insert(name.to_string(), func);
  }

  pub fn resolve_binding(&self, entity: &str, field: Option<&str>) -> Result<Option<Value>> {
    let entity_data = self.store.get(entity);
    match (entity_data, field) {
      (Some(data), Some(f)) => {
        let resolved = data
          .get(f)
          .cloned()
          .ok_or_else(|| AppError::NotFound(format!("Field '{}' on entity '{}'", f, entity)))?;
        Ok(Some(resolved))
      }
      (Some(data), None) => Ok(Some(data)),
      (None, _) => Err(AppError::NotFound(format!(
        "Entity '{}' not found in store",
        entity
      ))),
    }
  }

  pub fn has_function(&self, name: &str) -> bool {
    self.functions.contains_key(name)
  }

  pub fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Value> {
    self
      .functions
      .get(name)
      .ok_or_else(|| AppError::NotFound(format!("Function '{}' not found", name)))?(args)
  }
}
