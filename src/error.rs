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
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
  PlayerNotFound(String),
  GhostNotFound(String),
  SessionNotFound(String),
  InvalidPhase(String),
  BackupFailed(String),
  ProcessNotFound(String),
  ServiceNotFound(String),
  PathOutsideAllowed(String),
  Config(String),
  Voice(String),
  Settings(String),
  VLM(String),
  Music(String),
  Cache(String),
  Lock(String),
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
      AppError::PlayerNotFound(msg) => write!(f, "Player not found: {}", msg),
      AppError::GhostNotFound(msg) => write!(f, "Ghost not found: {}", msg),
      AppError::SessionNotFound(msg) => write!(f, "Session not found: {}", msg),
      AppError::InvalidPhase(msg) => write!(f, "Invalid phase: {}", msg),
      AppError::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
      AppError::ProcessNotFound(msg) => write!(f, "Process not found: {}", msg),
      AppError::ServiceNotFound(msg) => write!(f, "Service not found: {}", msg),
      AppError::PathOutsideAllowed(msg) => write!(f, "Path outside allowed: {}", msg),
      AppError::Config(msg) => write!(f, "Config error: {}", msg),
      AppError::Voice(msg) => write!(f, "Voice error: {}", msg),
      AppError::Settings(msg) => write!(f, "Settings error: {}", msg),
      AppError::VLM(msg) => write!(f, "VLM error: {}", msg),
      AppError::Music(msg) => write!(f, "Music error: {}", msg),
      AppError::Cache(msg) => write!(f, "Cache error: {}", msg),
      AppError::Lock(msg) => write!(f, "Lock error: {}", msg),
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

impl From<serde_json::Error> for AppError {
  fn from(err: serde_json::Error) -> Self {
    AppError::ValidationError(err.to_string())
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    AppError::Io(err.to_string())
  }
}

impl AppError {
  pub fn into_response<T>(self) -> crate::response::Response<T> {
    use crate::response::{Response, Status};
    match self {
      AppError::NotFound(entity) => Response {
        status: Status::NotFound,
        message: format!("{} not found", entity),
        data: None,
      },
      AppError::ValidationError(msg) => Response {
        status: Status::ValidationError,
        message: msg,
        data: None,
      },
      AppError::Duplicate(entity) => Response {
        status: Status::Duplicate,
        message: entity,
        data: None,
      },
      AppError::Unauthorized => Response {
        status: Status::Unauthorized,
        message: "Unauthorized".into(),
        data: None,
      },
      AppError::Forbidden => Response {
        status: Status::Forbidden,
        message: "Forbidden".into(),
        data: None,
      },
      AppError::Internal(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::Database(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::Network(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::Io(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::PermissionDenied(msg) => Response {
        status: Status::Forbidden,
        message: msg,
        data: None,
      },
      AppError::InvalidPath(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::RequestFailed(msg) => Response {
        status: Status::Error,
        message: msg,
        data: None,
      },
      AppError::PlayerNotFound(msg) => Response {
        status: Status::NotFound,
        message: format!("Player not found: {}", msg),
        data: None,
      },
      AppError::GhostNotFound(msg) => Response {
        status: Status::NotFound,
        message: format!("Ghost not found: {}", msg),
        data: None,
      },
      AppError::SessionNotFound(msg) => Response {
        status: Status::NotFound,
        message: format!("Session not found: {}", msg),
        data: None,
      },
      AppError::InvalidPhase(msg) => Response {
        status: Status::ValidationError,
        message: format!("Invalid phase: {}", msg),
        data: None,
      },
      AppError::BackupFailed(msg) => Response {
        status: Status::Error,
        message: format!("Backup failed: {}", msg),
        data: None,
      },
      AppError::ProcessNotFound(msg) => Response {
        status: Status::NotFound,
        message: format!("Process not found: {}", msg),
        data: None,
      },
      AppError::ServiceNotFound(msg) => Response {
        status: Status::NotFound,
        message: format!("Service not found: {}", msg),
        data: None,
      },
      AppError::PathOutsideAllowed(msg) => Response {
        status: Status::Forbidden,
        message: format!("Path outside allowed: {}", msg),
        data: None,
      },
      AppError::Config(msg) => Response {
        status: Status::Error,
        message: format!("Config error: {}", msg),
        data: None,
      },
      AppError::Voice(msg) => Response {
        status: Status::Error,
        message: format!("Voice error: {}", msg),
        data: None,
      },
      AppError::Settings(msg) => Response {
        status: Status::Error,
        message: format!("Settings error: {}", msg),
        data: None,
      },
      AppError::VLM(msg) => Response {
        status: Status::Error,
        message: format!("VLM error: {}", msg),
        data: None,
      },
      AppError::Music(msg) => Response {
        status: Status::Error,
        message: format!("Music error: {}", msg),
        data: None,
      },
      AppError::Cache(msg) => Response {
        status: Status::Error,
        message: format!("Cache error: {}", msg),
        data: None,
      },
      AppError::Lock(msg) => Response {
        status: Status::Error,
        message: format!("Lock error: {}", msg),
        data: None,
      },
    }
  }
}

#[derive(Debug)]
pub struct ProjectError {
  pub error: AppError,
  pub context: Option<String>,
  pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ProjectError {
  pub fn new(error: AppError) -> Self {
    Self {
      error,
      context: None,
      source: None,
    }
  }
  pub fn with_context(error: AppError, context: impl Into<String>) -> Self {
    Self {
      error,
      context: Some(context.into()),
      source: None,
    }
  }
  pub fn with_source(error: AppError, source: Box<dyn std::error::Error + Send + Sync>) -> Self {
    Self {
      error,
      context: None,
      source: Some(source),
    }
  }
}

impl std::fmt::Display for ProjectError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.context {
      Some(ctx) => write!(f, "{:?}: {}", self.error, ctx),
      None => write!(f, "{:?}", self.error),
    }
  }
}

impl std::error::Error for ProjectError {}

impl Clone for ProjectError {
  fn clone(&self) -> Self {
    Self {
      error: self.error.clone(),
      context: self.context.clone(),
      source: None,
    }
  }
}

impl From<AppError> for ProjectError {
  fn from(error: AppError) -> Self {
    Self::new(error)
  }
}
