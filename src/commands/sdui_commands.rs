use crate::Result;

#[tauri::command]
pub fn check_permission(_permission: String, _resource: String, _action: String) -> Result<bool> {
  Ok(true)
}
