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
        }
    }
}

impl std::error::Error for AppError {}

impl From<orm::OrmError> for AppError {
    fn from(err: orm::OrmError) -> Self {
        use orm::OrmError;
        match err {
            OrmError::NotFound(entity) => AppError::NotFound(entity),
            OrmError::ValidationError(msg) => AppError::ValidationError(msg),
            OrmError::Duplicate(entity) => AppError::Duplicate(entity),
            OrmError::Database(msg) => AppError::Database(msg),
            OrmError::Network(msg) => AppError::Network(msg),
            OrmError::Internal(msg) => AppError::Internal(msg),
        }
    }
}

pub mod orm {
    use thiserror::Error;

    #[derive(Debug, Clone, Error)]
    pub enum OrmError {
        #[error("{0} not found")]
        NotFound(String),
        #[error("Validation error: {0}")]
        ValidationError(String),
        #[error("{0} already exists")]
        Duplicate(String),
        #[error("Database error: {0}")]
        Database(String),
        #[error("Network error: {0}")]
        Network(String),
        #[error("Internal error: {0}")]
        Internal(String),
    }
}
