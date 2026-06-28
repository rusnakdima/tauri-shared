use crate::logger::{LogLevel, Logger};
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
