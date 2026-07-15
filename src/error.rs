#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_app_error_display_not_found() {
    let err = AppError::NotFound("User".to_string());
    assert_eq!(format!("{}", err), "User not found");
  }

  #[test]
  fn test_app_error_display_validation_error() {
    let err = AppError::ValidationError("Invalid email".to_string());
    assert_eq!(format!("{}", err), "Validation error: Invalid email");
  }

  #[test]
  fn test_app_error_display_duplicate() {
    let err = AppError::Duplicate("Username".to_string());
    assert_eq!(format!("{}", err), "Username already exists");
  }

  #[test]
  fn test_app_error_display_unauthorized() {
    let err = AppError::Unauthorized;
    assert_eq!(format!("{}", err), "Unauthorized");
  }

  #[test]
  fn test_app_error_display_forbidden() {
    let err = AppError::Forbidden;
    assert_eq!(format!("{}", err), "Forbidden");
  }

  #[test]
  fn test_app_error_display_internal() {
    let err = AppError::Internal("Unexpected error".to_string());
    assert_eq!(format!("{}", err), "Internal error: Unexpected error");
  }

  #[test]
  fn test_app_error_display_database() {
    let err = AppError::Database("Connection failed".to_string());
    assert_eq!(format!("{}", err), "Database error: Connection failed");
  }

  #[test]
  fn test_app_error_display_network() {
    let err = AppError::Network("Timeout".to_string());
    assert_eq!(format!("{}", err), "Network error: Timeout");
  }

  #[test]
  fn test_app_error_display_io() {
    let err = AppError::Io("File not found".to_string());
    assert_eq!(format!("{}", err), "IO error: File not found");
  }

  #[test]
  fn test_app_error_display_permission_denied() {
    let err = AppError::PermissionDenied("Access denied".to_string());
    assert_eq!(format!("{}", err), "Permission denied: Access denied");
  }

  #[test]
  fn test_app_error_display_invalid_path() {
    let err = AppError::InvalidPath("/invalid/../path".to_string());
    assert_eq!(format!("{}", err), "Invalid path: /invalid/../path");
  }

  #[test]
  fn test_app_error_display_request_failed() {
    let err = AppError::RequestFailed("HTTP 500".to_string());
    assert_eq!(format!("{}", err), "Request failed: HTTP 500");
  }

  #[test]
  fn test_app_error_from_orm_not_found() {
    use nosql_orm::error::OrmError;
    let orm_err = OrmError::NotFound("Document".to_string());
    let app_err: AppError = orm_err.into();
    assert_eq!(format!("{}", app_err), "Document not found");
  }

  #[test]
  fn test_app_error_from_orm_validation() {
    use nosql_orm::error::OrmError;
    let orm_err = OrmError::Validation("Invalid schema".to_string());
    let app_err: AppError = orm_err.into();
    assert_eq!(format!("{}", app_err), "Validation error: Invalid schema");
  }

  #[test]
  fn test_app_error_from_orm_duplicate() {
    use nosql_orm::error::OrmError;
    let orm_err = OrmError::Duplicate("Record".to_string());
    let app_err: AppError = orm_err.into();
    assert_eq!(format!("{}", app_err), "Record already exists");
  }

  #[test]
  fn test_app_error_from_orm_connection() {
    use nosql_orm::error::OrmError;
    let orm_err = OrmError::Connection("Connection refused".to_string());
    let app_err: AppError = orm_err.into();
    assert_eq!(format!("{}", app_err), "Database error: Connection refused");
  }

  #[test]
  fn test_app_error_from_orm_internal() {
    use nosql_orm::error::OrmError;
    let orm_err = OrmError::Internal("Orm internal".to_string());
    let app_err: AppError = orm_err.into();
    assert_eq!(format!("{}", app_err), "Internal error: Orm internal");
  }

  #[test]
  fn test_app_error_from_tungstenite() {
    use tokio_tungstenite::tungstenite::Error;
    let ws_err = Error::ConnectionClosed;
    let app_err: AppError = ws_err.into();
    assert_eq!(
      format!("{}", app_err),
      "Network error: Connection closed normally"
    );
  }

  #[test]
  fn test_app_error_debug() {
    let err = AppError::NotFound("User".to_string());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("NotFound"));
  }

  #[test]
  fn test_app_error_serialization() {
    let err = AppError::NotFound("User".to_string());
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("notFound"));
    assert!(json.contains("User"));
  }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppError {
  NotFound(String),
  ValidationError(String),
  Duplicate(String),
  Unauthorized,
  Forbidden,
  Internal(String),
  Database(String),
  Network(String),
  Io(String),
  PermissionDenied(String),
  InvalidPath(String),
  RequestFailed(String),
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AppError::NotFound(entity) => write!(f, "{} not found", entity),
      AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
      AppError::Duplicate(entity) => write!(f, "{} already exists", entity),
      AppError::Unauthorized => write!(f, "Unauthorized"),
      AppError::Forbidden => write!(f, "Forbidden"),
      AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
      AppError::Database(msg) => write!(f, "Database error: {}", msg),
      AppError::Network(msg) => write!(f, "Network error: {}", msg),
      AppError::Io(msg) => write!(f, "IO error: {}", msg),
      AppError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
      AppError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
      AppError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
    }
  }
}

impl std::error::Error for AppError {}

impl From<tokio_tungstenite::tungstenite::Error> for AppError {
  fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
    AppError::Network(err.to_string())
  }
}

impl From<nosql_orm::error::OrmError> for AppError {
  fn from(err: nosql_orm::error::OrmError) -> Self {
    match err {
      nosql_orm::error::OrmError::NotFound(entity) => AppError::NotFound(entity),
      nosql_orm::error::OrmError::Validation(msg) => AppError::ValidationError(msg),
      nosql_orm::error::OrmError::Duplicate(entity) => AppError::Duplicate(entity),
      nosql_orm::error::OrmError::Connection(msg) => AppError::Database(msg),
      nosql_orm::error::OrmError::Provider(msg) => AppError::Database(msg),
      nosql_orm::error::OrmError::Query(msg) => AppError::Database(msg),
      nosql_orm::error::OrmError::Internal(msg) => AppError::Internal(msg),
      _ => AppError::Internal(err.to_string()),
    }
  }
}
