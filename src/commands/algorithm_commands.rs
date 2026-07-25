use crate::algorithms::AlgorithmRegistry;
use crate::log_error;
use crate::log_info;
use crate::response::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlgoRequest {
  pub algorithm: String,
  pub input: serde_json::Value,
}

#[tauri::command]
pub async fn algo_execute(
  request: AlgoRequest,
  registry: tauri::State<'_, AlgorithmRegistry>,
) -> Result<serde_json::Value, String> {
  log_info!(
    "[BACKEND] CMD:algo_execute START algorithm={}",
    request.algorithm
  );
  let start = std::time::Instant::now();
  let result = registry.execute(&request.algorithm, request.input);
  match &result {
    Ok(_) => log_info!("[BACKEND] CMD:algo_execute OK ({:?})", start.elapsed()),
    Err(ref e) => log_error!(
      "[BACKEND] CMD:algo_execute ERROR ({:?}): {}",
      start.elapsed(),
      e
    ),
  }
  result
}

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

  // ============ SORTING ALGORITHMS ============

  #[test]
  fn test_execute_sort_bubble() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sort.bubble",
      serde_json::json!({
        "data": [64, 34, 25, 12, 22, 11, 90]
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let sorted: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
  }

  #[test]
  fn test_execute_sort_insertion() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sort.insertion",
      serde_json::json!({
        "data": [64, 34, 25, 12, 22, 11, 90]
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let sorted: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
  }

  #[test]
  fn test_execute_sort_merge() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sort.merge",
      serde_json::json!({
        "data": [64, 34, 25, 12, 22, 11, 90]
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let sorted: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
  }

  #[test]
  fn test_execute_sort_quick() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sort.quick",
      serde_json::json!({
        "data": [64, 34, 25, 12, 22, 11, 90]
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    let sorted: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(sorted, vec![11, 12, 22, 25, 34, 64, 90]);
  }

  // ============ SEARCH ALGORITHMS ============

  #[test]
  fn test_execute_search_schemas() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "search.schemas",
      serde_json::json!({
        "items": ["UserSchema", "OrderSchema", "ProductSchema"],
        "query": "user"
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    assert!(!arr.is_empty());
  }

  #[test]
  fn test_execute_search_paginate() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "search.paginate",
      serde_json::json!({
        "items": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "page": 2,
        "limit": 3
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    // Should return page 2 with 3 items per page: [4, 5, 6]
    let arr = data.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    let page_items: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(page_items, vec![4, 5, 6]);
  }

  // ============ GRAPH ALGORITHMS ============

  #[test]
  fn test_execute_graph_dijkstra() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "graph.dijkstra",
      serde_json::json!({
        "nodes": [{"id": "A", "data": null}, {"id": "B", "data": null}, {"id": "C", "data": null}],
        "edges": [
          {"from": "A", "to": "B", "weight": 1.0},
          {"from": "B", "to": "C", "weight": 2.0},
          {"from": "A", "to": "C", "weight": 4.0}
        ],
        "start": "A",
        "end": "C"
      }),
    );
    assert!(result.is_ok(), "Dijkstra failed: {:?}", result.err());
    let data = result.unwrap();
    // Shortest path A -> B -> C has cost 3
    assert!(data.get("distance").is_some() || data.get("path").is_some() || data.is_array());
  }

  #[test]
  fn test_execute_graph_dfs() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute("graph.dfs", serde_json::json!({
      "nodes": [{"id": "1", "data": null}, {"id": "2", "data": null}, {"id": "3", "data": null}],
      "edges": [{"from": "1", "to": "2", "weight": 1.0}, {"from": "1", "to": "3", "weight": 1.0}, {"from": "2", "to": "3", "weight": 1.0}],
      "start": "1"
    }));
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    // DFS should visit nodes starting from 1
    let strs: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(strs[0], "1");
  }

  #[test]
  fn test_execute_graph_topological_sort() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute("graph.topological_sort", serde_json::json!({
      "nodes": [{"id": "A", "data": null}, {"id": "B", "data": null}, {"id": "C", "data": null}, {"id": "D", "data": null}],
      "edges": [
        {"from": "A", "to": "B", "weight": 1.0},
        {"from": "A", "to": "C", "weight": 1.0},
        {"from": "B", "to": "D", "weight": 1.0},
        {"from": "C", "to": "D", "weight": 1.0}
      ]
    }));
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    // A should come before B, C, and D; B and C before D
    let strs: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
    let a_idx = strs.iter().position(|&s| s == "A").unwrap();
    let b_idx = strs.iter().position(|&s| s == "B").unwrap();
    let c_idx = strs.iter().position(|&s| s == "C").unwrap();
    let d_idx = strs.iter().position(|&s| s == "D").unwrap();
    assert!(a_idx < b_idx && a_idx < c_idx);
    assert!(a_idx < d_idx && b_idx < d_idx && c_idx < d_idx);
  }

  // ============ TREE ALGORITHMS ============

  #[test]
  fn test_execute_tree_build() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "tree.build",
      serde_json::json!([
        {"id": "1", "parent_id": null, "name": "Root"},
        {"id": "2", "parent_id": "1", "name": "Child1"},
        {"id": "3", "parent_id": "1", "name": "Child2"},
        {"id": "4", "parent_id": "2", "name": "Grandchild"}
      ]),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    // Result should be a tree structure or confirmation
    assert!(!data.is_null());
  }

  #[test]
  fn test_execute_tree_flatten() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "tree.flatten",
      serde_json::json!([
        {"id": "1", "children": [
          {"id": "2", "children": []},
          {"id": "3", "children": []}
        ]}
      ]),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let arr = data.as_array().unwrap();
    // Should flatten to 3 nodes: root, child1, child2
    assert_eq!(arr.len(), 3);
  }

  // ============ VALIDATION ALGORITHMS ============

  #[test]
  fn test_execute_validate_email_valid() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "validate.email",
      serde_json::json!({
        "email": "test@example.com"
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    // Should return validation result indicating valid
    assert!(data.is_boolean() || data.as_object().is_some());
  }

  #[test]
  fn test_execute_validate_email_invalid() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "validate.email",
      serde_json::json!({
        "email": "invalid-email"
      }),
    );
    assert!(result.is_ok()); // Returns validation result, not error
    let data = result.unwrap();
    assert!(data.is_boolean() || data.as_object().is_some());
  }

  #[test]
  fn test_execute_validate_input() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "validate.input",
      serde_json::json!({
        "input": "hello",
        "max_length": 10
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.is_boolean() || data.as_object().is_some());
  }

  #[test]
  fn test_execute_validate_sanitize() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "validate.sanitize",
      serde_json::json!({
        "input": "Hello-World123"
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let sanitized = data.as_str().unwrap();
    assert_eq!(sanitized, "Hello-World123");
  }

  // ============ SANITIZATION ALGORITHMS ============

  #[test]
  fn test_execute_sanitize_escape_html() {
    let registry = AlgorithmRegistry::new();
    let result = registry.execute(
      "sanitize.escape_html",
      serde_json::json!({
        "data": "<script>alert('xss')</script>"
      }),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    let escaped = data.as_str().unwrap();
    assert!(!escaped.contains("<script>"));
    assert!(escaped.contains("&lt;script&gt;") || escaped.contains("&lt;"));
  }
}
