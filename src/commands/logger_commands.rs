use crate::log_info;
use crate::logger::{LogEntry, LogLevel, Logger};
use crate::response::Response;
use crate::AppError;
use crate::Result;

#[tauri::command]
pub fn write_log_to_file(
  level: String,
  message: String,
  source: Option<String>,
) -> Result<Response<()>> {
  log_info!("[BACKEND] CMD:write_log_to_file START level={}", level);
  let log_level = match level.to_lowercase().as_str() {
    "debug" => LogLevel::Debug,
    "info" => LogLevel::Info,
    "warn" | "warning" => LogLevel::Warn,
    "error" => LogLevel::Error,
    _ => {
      log_info!(
        "[BACKEND] CMD:write_log_to_file ERROR: Invalid log level {}",
        level
      );
      return Err(AppError::ValidationError(format!("Invalid log level: {}", level)).into());
    }
  };

  Logger::global().log(log_level, &message, source.as_deref());
  log_info!("[BACKEND] CMD:write_log_to_file OK");
  Ok(Response::success((), Some("Log written to file")))
}

/// Set the global minimum log level dynamically
/// Levels: "debug" | "info" | "warn" | "error"
#[tauri::command]
pub fn set_log_level(level: String) -> Result<Response<String>> {
  log_info!("[BACKEND] CMD:set_log_level START level={}", level);
  let log_level = LogLevel::from_str(&level).ok_or_else(|| {
    log_info!(
      "[BACKEND] CMD:set_log_level ERROR: Invalid log level {}",
      level
    );
    AppError::ValidationError(format!("Invalid log level: {}", level))
  })?;

  Logger::set_level(log_level);
  let current = Logger::get_level();
  let level_str = current.as_str().to_string();
  log_info!("[BACKEND] CMD:set_log_level OK level={}", level_str);

  Ok(Response::success(
    level_str,
    Some(&format!("Log level set to {}", current.as_str())),
  ))
}

/// Get the current global minimum log level
#[tauri::command]
pub fn get_log_level() -> Result<Response<String>> {
  log_info!("[BACKEND] CMD:get_log_level START");
  let current = Logger::get_level();
  log_info!("[BACKEND] CMD:get_log_level OK level={}", current.as_str());
  Ok(Response::success(
    current.as_str().to_string(),
    Some("Current log level"),
  ))
}

/// Get all stored log entries (respects current minimum level)
#[tauri::command]
pub fn get_log_entries() -> Result<Response<Vec<LogEntry>>> {
  log_info!("[BACKEND] CMD:get_log_entries START");
  let entries = Logger::global().get_entries();
  log_info!("[BACKEND] CMD:get_log_entries OK count={}", entries.len());
  Ok(Response::success(entries, Some("Log entries retrieved")))
}

/// Clear all stored log entries
#[tauri::command]
pub fn clear_logs() -> Result<Response<()>> {
  log_info!("[BACKEND] CMD:clear_logs START");
  Logger::global().clear();
  log_info!("[BACKEND] CMD:clear_logs OK");
  Ok(Response::success((), Some("Logs cleared")))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_write_log_info() {
    let result = write_log_to_file("info".to_string(), "test info message".to_string(), None);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_write_log_error() {
    let result = write_log_to_file("error".to_string(), "test error message".to_string(), None);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_set_and_get_log_level() {
    let result = set_log_level("debug".to_string());
    assert!(result.is_ok());
    let get_result = get_log_level();
    assert!(get_result.is_ok());
  }

  #[tokio::test]
  async fn test_get_log_entries() {
    let _ = write_log_to_file("info".to_string(), "test".to_string(), None);
    let result = get_log_entries();
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_clear_logs() {
    let result = clear_logs();
    assert!(result.is_ok());
  }
}
