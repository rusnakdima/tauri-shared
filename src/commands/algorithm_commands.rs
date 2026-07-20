use crate::algorithms::AlgorithmRegistry;
use crate::response::Response;

#[tauri::command]
pub fn execute_algorithm(
  name: String,
  input: serde_json::Value,
  registry: tauri::State<'_, AlgorithmRegistry>,
) -> Response<serde_json::Value> {
  match registry.execute(&name, input) {
    Ok(data) => Response::success(data, None),
    Err(err) => Response::error(err),
  }
}

#[tauri::command]
pub fn list_algorithms(registry: tauri::State<'_, AlgorithmRegistry>) -> Response<Vec<String>> {
  Response::success(registry.list(), None)
}
