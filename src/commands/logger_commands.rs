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
  let log_level = match level.to_lowercase().as_str() {
    "debug" => LogLevel::Debug,
    "info" => LogLevel::Info,
    "warn" | "warning" => LogLevel::Warn,
    "error" => LogLevel::Error,
    _ => return Err(AppError::ValidationError(format!("Invalid log level: {}", level)).into()),
  };

  Logger::global().log(log_level, &message, source.as_deref());

  Ok(Response::success((), Some("Log written to file")))
}

/// Set the global minimum log level dynamically
/// Levels: "debug" | "info" | "warn" | "error"
#[tauri::command]
pub fn set_log_level(level: String) -> Result<Response<String>> {
  let log_level = LogLevel::from_str(&level)
    .ok_or_else(|| AppError::ValidationError(format!("Invalid log level: {}", level)))?;

  Logger::set_level(log_level);
  let current = Logger::get_level();
  let level_str = current.as_str().to_string();

  Ok(Response::success(
    level_str,
    Some(&format!("Log level set to {}", current.as_str())),
  ))
}

/// Get the current global minimum log level
#[tauri::command]
pub fn get_log_level() -> Result<Response<String>> {
  let current = Logger::get_level();
  Ok(Response::success(
    current.as_str().to_string(),
    Some("Current log level"),
  ))
}

/// Get all stored log entries (respects current minimum level)
#[tauri::command]
pub fn get_log_entries() -> Result<Response<Vec<LogEntry>>> {
  let entries = Logger::global().get_entries();
  Ok(Response::success(entries, Some("Log entries retrieved")))
}

/// Clear all stored log entries
#[tauri::command]
pub fn clear_logs() -> Result<Response<()>> {
  Logger::global().clear();
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
