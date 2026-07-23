use crate::algorithms::AlgorithmRegistry;
use crate::log_error;
use crate::log_info;
use crate::response::Response;

#[tauri::command]
pub fn execute_algorithm(
  name: String,
  input: serde_json::Value,
  registry: tauri::State<'_, AlgorithmRegistry>,
) -> Response<serde_json::Value> {
  log_info!("[BACKEND] CMD:execute_algorithm START name={}", name);
  let start = std::time::Instant::now();
  let result = match registry.execute(&name, input) {
    Ok(data) => {
      log_info!("[BACKEND] CMD:execute_algorithm OK ({:?})", start.elapsed());
      Response::success(data, None)
    }
    Err(err) => {
      log_error!(
        "[BACKEND] CMD:execute_algorithm ERROR ({:?}): {}",
        start.elapsed(),
        err
      );
      Response::error(err)
    }
  };
  result
}

#[tauri::command]
pub fn list_algorithms(registry: tauri::State<'_, AlgorithmRegistry>) -> Response<Vec<String>> {
  log_info!("[BACKEND] CMD:list_algorithms START");
  let result = Response::success(registry.list(), None);
  let count = result.data.as_ref().map(|v| v.len()).unwrap_or(0);
  log_info!("[BACKEND] CMD:list_algorithms OK count={}", count);
  result
}

#[cfg(test)]
mod tests {
  use crate::algorithms::AlgorithmRegistry;

  #[test]
  fn test_list_algorithms_via_registry_directly() {
    let registry = AlgorithmRegistry::new();
    let algorithms = registry.list();
    assert!(!algorithms.is_empty());
  }

  #[test]
  fn test_execute_bubble_sort_via_registry_directly() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sort.bubble",
      serde_json::json!({ "data": [5, 3, 8, 1, 9] }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let sorted: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(sorted, vec![1, 3, 5, 8, 9]);
  }

  #[test]
  fn test_execute_bfs_via_registry_directly() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "graph.bfs",
      serde_json::json!({
        "nodes": [{"id": "1", "data": null}, {"id": "2", "data": null}, {"id": "3", "data": null}, {"id": "4", "data": null}],
        "edges": [{"from": "1", "to": "2", "weight": 1}, {"from": "1", "to": "3", "weight": 1}, {"from": "2", "to": "4", "weight": 1}],
        "start": "1"
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let strs: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(strs, vec!["1", "2", "3", "4"]);
  }

  #[test]
  fn test_execute_unknown_algorithm_returns_err() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute("nonexistent_algo", serde_json::json!(null));
    assert!(result.is_err());
  }
}
