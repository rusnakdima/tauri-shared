use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T> {
  pub status: String,
  pub message: Option<String>,
  pub data: Option<T>,
  pub timestamp: i64,
}

use reqwest::Client;
use std::time::Duration;

pub struct HttpClient {
  pub base_url: String,
  client: Client,
}

impl HttpClient {
  pub fn new(base_url: String) -> Self {
    let client = Client::builder()
      .timeout(Duration::from_secs(30))
      .build()
      .expect("Failed to create HTTP client");

    Self { base_url, client }
  }

  pub async fn get<T>(&self, path: String) -> Result<HttpResponse<T>>
  where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let url = format!("{}{}", self.base_url, path);
    let response = self
      .client
      .get(&url)
      .send()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;

    if !response.status().is_success() {
      return Err(AppError::RequestFailed(format!(
        "HTTP error: {:?}",
        response.status()
      )));
    }

    let text = response
      .text()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;
    let json: HttpResponse<T> =
      serde_json::from_str(&text).map_err(|e| AppError::RequestFailed(e.to_string()))?;

    Ok(json)
  }

  pub async fn post<T>(&self, path: String, body: Value) -> Result<HttpResponse<T>>
  where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let url = format!("{}{}", self.base_url, path);
    let response = self
      .client
      .post(&url)
      .json(&body)
      .send()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;

    if !response.status().is_success() {
      return Err(AppError::RequestFailed(format!(
        "HTTP error: {:?}",
        response.status()
      )));
    }

    let text = response
      .text()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;
    let json: HttpResponse<T> =
      serde_json::from_str(&text).map_err(|e| AppError::RequestFailed(e.to_string()))?;

    Ok(json)
  }

  pub async fn put<T>(&self, path: String, body: Value) -> Result<HttpResponse<T>>
  where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let url = format!("{}{}", self.base_url, path);
    let response = self
      .client
      .put(&url)
      .json(&body)
      .send()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;

    if !response.status().is_success() {
      return Err(AppError::RequestFailed(format!(
        "HTTP error: {:?}",
        response.status()
      )));
    }

    let text = response
      .text()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;
    let json: HttpResponse<T> =
      serde_json::from_str(&text).map_err(|e| AppError::RequestFailed(e.to_string()))?;

    Ok(json)
  }

  pub async fn delete<T>(&self, path: String) -> Result<HttpResponse<T>>
  where
    T: for<'de> Deserialize<'de> + Serialize,
  {
    let url = format!("{}{}", self.base_url, path);
    let response = self
      .client
      .delete(&url)
      .send()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;

    if !response.status().is_success() {
      return Err(AppError::RequestFailed(format!(
        "HTTP error: {:?}",
        response.status()
      )));
    }

    let text = response
      .text()
      .await
      .map_err(|e| AppError::RequestFailed(e.to_string()))?;
    let json: HttpResponse<T> =
      serde_json::from_str(&text).map_err(|e| AppError::RequestFailed(e.to_string()))?;

    Ok(json)
  }
}

#[derive(ThisError, Debug)]
pub enum AppError {
  #[error("HTTP error: {0}")]
  HttpError(String),
  #[error("JSON error: {0}")]
  JsonError(#[from] serde_json::Error),
  #[error("Request failed: {0}")]
  RequestFailed(String),
}
