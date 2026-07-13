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

pub use nosql_orm::error::OrmResult;
