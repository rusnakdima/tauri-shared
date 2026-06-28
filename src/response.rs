use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Success,
    Created,
    Updated,
    Deleted,
    Error,
    ValidationError,
    NotFound,
    Unauthorized,
    Forbidden,
    Info,
    Warning,
    Duplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub status: Status,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T, message: Option<&str>) -> Self {
        Self {
            status: Status::Success,
            message: message.map(String::from).unwrap_or_default(),
            data: Some(data),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status: Status::Created,
            message: String::new(),
            data: Some(data),
        }
    }

    pub fn updated(data: T) -> Self {
        Self {
            status: Status::Updated,
            message: String::new(),
            data: Some(data),
        }
    }

    pub fn deleted(data: T) -> Self {
        Self {
            status: Status::Deleted,
            message: String::new(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: Status::Error,
            message: message.into(),
            data: None,
        }
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self {
            status: Status::ValidationError,
            message: message.into(),
            data: None,
        }
    }

    pub fn not_found(entity: impl Into<String>) -> Self {
        Self {
            status: Status::NotFound,
            message: format!("{} not found", entity.into()),
            data: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: Status::Unauthorized,
            message: message.into(),
            data: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: Status::Forbidden,
            message: message.into(),
            data: None,
        }
    }
}

impl<T: Clone> Response<T> {
    pub fn map_data<U: Clone>(self, f: impl FnOnce(T) -> U) -> Response<U> {
        Response {
            status: self.status,
            message: self.message,
            data: self.data.map(f),
        }
    }
}
