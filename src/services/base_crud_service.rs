//! BaseCrudService - shared CRUD service for settings and commands persistence.
//!
//! This module provides a unified service for basic CRUD operations on JSON-backed
//! storage, used by VoiceAssistant and potentially other apps.

use crate::Logger;
use nosql_orm::prelude::*;
use nosql_orm::validators::Validate;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Builder for constructing a BaseCrudService with optional logger.
#[derive(Default)]
pub struct BaseCrudServiceBuilder {
  logger: Option<Arc<Logger>>,
}

impl BaseCrudServiceBuilder {
  pub fn new() -> Self {
    Self { logger: None }
  }

  pub fn with_logger(mut self, logger: Arc<Logger>) -> Self {
    self.logger = Some(logger);
    self
  }

  pub fn build(self, json_provider: JsonProvider) -> BaseCrudService {
    BaseCrudService {
      json_provider,
      logger: self.logger,
    }
  }
}

/// Shared CRUD service for basic entity persistence.
///
/// Provides typed get/save operations for entities like settings and commands
/// that are stored as single records or collections in JSON storage.
///
/// # Type Parameters
/// - `T`: Entity type that implements `Model` (from nosql_orm)
///
/// # Example
/// ```
/// use tauri_shared::services::BaseCrudService;
/// use nosql_orm::prelude::*;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Model)]
/// #[table_name("settings")]
/// struct SettingsEntity {
///   id: Option<String>,
///   theme: String,
/// }
/// ```
pub struct BaseCrudService {
  json_provider: JsonProvider,
  logger: Option<Arc<Logger>>,
}

impl Clone for BaseCrudService {
  fn clone(&self) -> Self {
    Self {
      json_provider: self.json_provider.clone(),
      logger: self.logger.clone(),
    }
  }
}

impl BaseCrudService {
  /// Create a new BaseCrudService without logging.
  pub fn new(json_provider: JsonProvider) -> Self {
    Self {
      json_provider,
      logger: None,
    }
  }

  /// Create a new BaseCrudService with a builder for configuration.
  pub fn builder(json_provider: JsonProvider) -> Self {
    Self::new(json_provider)
  }

  /// Create a new BaseCrudService with logging enabled.
  pub fn with_logger(json_provider: JsonProvider, logger: Arc<Logger>) -> Self {
    Self {
      json_provider,
      logger: Some(logger),
    }
  }

  fn log_debug(&self, message: &str) {
    if let Some(ref logger) = self.logger {
      logger.debug(message);
    }
  }

  fn log_error(&self, message: &str) {
    if let Some(ref logger) = self.logger {
      logger.error(message);
    }
  }

  /// Get the JSON provider used by this service.
  pub fn json_provider(&self) -> &JsonProvider {
    &self.json_provider
  }

  /// Get a single entity by table name, returning the first record.
  ///
  /// Returns `Ok(None)` if no records exist in the table.
  pub async fn get_first<T: Entity + Validate + Send + Sync>(&self) -> Result<Option<T>, String> {
    self.log_debug(&format!(
      "BaseCrudService: Getting first from {}",
      T::table_name()
    ));
    let repo: Repository<T, _> = Repository::new(self.json_provider.clone());
    let result = repo.find_all().await.map_err(|e| e.to_string())?;
    self.log_debug(&format!(
      "BaseCrudService: Got {} records from {}",
      result.len(),
      T::table_name()
    ));
    Ok(result.into_iter().next())
  }

  /// Get all entities by table name.
  pub async fn get_all<T: Entity + Validate + Send + Sync>(&self) -> Result<Vec<T>, String> {
    self.log_debug(&format!(
      "BaseCrudService: Getting all from {}",
      T::table_name()
    ));
    let repo: Repository<T, _> = Repository::new(self.json_provider.clone());
    repo.find_all().await.map_err(|e| {
      self.log_error(&format!(
        "BaseCrudService: Failed to get all from {}: {}",
        T::table_name(),
        e
      ));
      e.to_string()
    })
  }

  /// Save a single entity (insert or update).
  ///
  /// For entities with `id: None`, a new record is created.
  /// For entities with an `id`, the record is updated.
  pub async fn save<T: Entity + Validate + Serialize + Send + Sync>(
    &self,
    entity: T,
  ) -> Result<T, String> {
    self.log_debug(&format!("BaseCrudService: Saving to {}", T::table_name()));
    let repo: Repository<T, _> = Repository::new(self.json_provider.clone());
    repo.save(entity).await.map_err(|e| {
      self.log_error(&format!(
        "BaseCrudService: Failed to save to {}: {}",
        T::table_name(),
        e
      ));
      e.to_string()
    })
  }

  /// Delete all records from a table.
  /// Returns the number of records deleted.
  pub async fn delete_all<T: Entity + Validate + Send + Sync>(&self) -> Result<usize, String> {
    self.log_debug(&format!(
      "BaseCrudService: Deleting all from {}",
      T::table_name()
    ));
    let repo: Repository<T, _> = Repository::new(self.json_provider.clone());
    repo.delete_many(None).await.map_err(|e| {
      self.log_error(&format!(
        "BaseCrudService: Failed to delete from {}: {}",
        T::table_name(),
        e
      ));
      e.to_string()
    })
  }

  /// Save a collection of entities, replacing all existing records.
  ///
  /// This first deletes all existing records in the table, then inserts
  /// each entity from the provided collection.
  pub async fn save_all<T: Entity + Validate + Serialize + Send + Sync>(
    &self,
    entities: Vec<T>,
  ) -> Result<(), String> {
    let count = entities.len();
    self.log_debug(&format!(
      "BaseCrudService: Saving {} entities to {}",
      count,
      T::table_name()
    ));
    let repo: Repository<T, _> = Repository::new(self.json_provider.clone());
    repo.delete_many(None).await.map_err(|e| {
      self.log_error(&format!(
        "BaseCrudService: Failed to clear {}: {}",
        T::table_name(),
        e
      ));
      e.to_string()
    })?;
    for entity in entities {
      repo.save(entity).await.map_err(|e| {
        self.log_error(&format!(
          "BaseCrudService: Failed to save entity to {}: {}",
          T::table_name(),
          e
        ));
        e.to_string()
      })?;
    }
    self.log_debug(&format!(
      "BaseCrudService: Successfully saved {} entities to {}",
      count,
      T::table_name()
    ));
    Ok(())
  }
}
