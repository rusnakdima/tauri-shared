use crate::algorithms::AlgorithmRegistry;

#[tauri::command]
pub fn execute_algorithm(
  name: String,
  input: serde_json::Value,
  registry: tauri::State<'_, AlgorithmRegistry>,
) -> Result<serde_json::Value, String> {
  registry.execute(&name, input)
}

#[tauri::command]
pub fn list_algorithms(
  registry: tauri::State<'_, AlgorithmRegistry>,
) -> Result<Vec<String>, String> {
  Ok(registry.list())
}
