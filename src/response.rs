#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_response_success() {
    let resp = Response::success("data", Some("success message"));
    assert_eq!(resp.status, Status::Success);
    assert_eq!(resp.message, "success message");
    assert_eq!(resp.data, Some("data"));
  }

  #[test]
  fn test_response_success_no_message() {
    let resp = Response::<&str>::success("data", None);
    assert_eq!(resp.status, Status::Success);
    assert_eq!(resp.message, "");
    assert_eq!(resp.data, Some("data"));
  }

  #[test]
  fn test_response_created() {
    let resp = Response::created("new-item");
    assert_eq!(resp.status, Status::Created);
    assert_eq!(resp.message, "");
    assert_eq!(resp.data, Some("new-item"));
  }

  #[test]
  fn test_response_updated() {
    let resp = Response::updated("updated-item");
    assert_eq!(resp.status, Status::Updated);
    assert_eq!(resp.message, "");
    assert_eq!(resp.data, Some("updated-item"));
  }

  #[test]
  fn test_response_deleted() {
    let resp = Response::deleted("deleted-item");
    assert_eq!(resp.status, Status::Deleted);
    assert_eq!(resp.message, "");
    assert_eq!(resp.data, Some("deleted-item"));
  }

  #[test]
  fn test_response_error() {
    let resp: Response<()> = Response::error("Something went wrong");
    assert_eq!(resp.status, Status::Error);
    assert_eq!(resp.message, "Something went wrong");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_error_with_data() {
    let resp = Response::error_with_data("error data", "Error occurred");
    assert_eq!(resp.status, Status::Error);
    assert_eq!(resp.message, "Error occurred");
    assert_eq!(resp.data, Some("error data"));
  }

  #[test]
  fn test_response_validation_error() {
    let resp: Response<()> = Response::validation_error("Invalid input");
    assert_eq!(resp.status, Status::ValidationError);
    assert_eq!(resp.message, "Invalid input");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_not_found() {
    let resp: Response<()> = Response::not_found("User");
    assert_eq!(resp.status, Status::NotFound);
    assert_eq!(resp.message, "User not found");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_unauthorized() {
    let resp: Response<()> = Response::unauthorized("Not authenticated");
    assert_eq!(resp.status, Status::Unauthorized);
    assert_eq!(resp.message, "Not authenticated");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_forbidden() {
    let resp: Response<()> = Response::forbidden("Access denied");
    assert_eq!(resp.status, Status::Forbidden);
    assert_eq!(resp.message, "Access denied");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_error_with_status() {
    let resp: Response<()> = Response::error_with_status(Status::Info, "Info message");
    assert_eq!(resp.status, Status::Info);
    assert_eq!(resp.message, "Info message");
    assert_eq!(resp.data, None);
  }

  #[test]
  fn test_response_map_data() {
    let resp = Response::success(42, None);
    let mapped = resp.map_data(|x| x * 2);
    assert_eq!(mapped.status, Status::Success);
    assert_eq!(mapped.data, Some(84));
  }

  #[test]
  fn test_response_map_data_none() {
    let resp: Response<i32> = Response::error("error");
    let mapped = resp.map_data(|x| x * 2);
    assert_eq!(mapped.status, Status::Error);
    assert_eq!(mapped.data, None);
  }

  #[test]
  fn test_response_clone() {
    let resp = Response::success("data", Some("msg"));
    let cloned = resp.clone();
    assert_eq!(cloned.status, resp.status);
    assert_eq!(cloned.message, resp.message);
    assert_eq!(cloned.data, resp.data);
  }

  #[test]
  fn test_status_serialization() {
    let status = Status::Success;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"success\"");
  }

  #[test]
  fn test_response_serialization() {
    let resp = Response::success("test-data", Some("message"));
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("\"status\":\"success\""));
    assert!(json.contains("\"message\":\"message\""));
    assert!(json.contains("\"data\":\"test-data\""));
  }
}

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

  pub fn error_with_data(data: T, message: impl Into<String>) -> Self {
    Self {
      status: Status::Error,
      message: message.into(),
      data: Some(data),
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

  pub fn error_with_status(status: Status, message: impl Into<String>) -> Self {
    Self {
      status,
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
