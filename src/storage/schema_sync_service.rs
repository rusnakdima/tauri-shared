//! SchemaSyncService - syncs schemas from MongoDB cloud to local JSON file.

use crate::log_info;
use nosql_orm::prelude::*;
use nosql_orm::providers::mongo::MongoProvider;

/// SchemaSyncService syncs schemas from MongoDB cloud to local JSON file.
/// Local file path is managed by the service (typically {app_data_dir}/{schema_id}_schema.json)
pub struct SchemaSyncService {
  /// Path to local JSON file, e.g., {app_data_dir}/translator_schema.json
  local_path: std::path::PathBuf,
  /// MongoDB remote provider
  remote: MongoProvider,
}

impl SchemaSyncService {
  /// Create new SchemaSyncService
  /// - local_path: Full path to local JSON file (e.g., /app_data/translator_schema.json)
  /// - mongo_uri: MongoDB connection URI (e.g., mongodb://localhost:27017)
  /// - db_name: MongoDB database name (e.g., schemas)
  pub async fn new(
    local_path: std::path::PathBuf,
    mongo_uri: &str,
    db_name: &str,
  ) -> OrmResult<Self> {
    // Ensure parent directory exists
    if let Some(parent) = local_path.parent() {
      std::fs::create_dir_all(parent)?;
    }

    let remote = MongoProvider::connect(mongo_uri, db_name).await?;

    Ok(Self { local_path, remote })
  }

  /// Sync schema from MongoDB to local JSON file
  /// Returns the schema after syncing
  pub async fn sync_schema(&self, schema_id: &str) -> OrmResult<serde_json::Value> {
    // Fetch from MongoDB
    let schema = self
      .remote
      .find_by_id("schemas", schema_id)
      .await?
      .ok_or_else(|| OrmError::NotFound(format!("Schema {} not found in MongoDB", schema_id)))?;

    // Save to local JSON file
    if let Some(parent) = self.local_path.parent() {
      std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&schema)?;
    std::fs::write(&self.local_path, json)?;

    log_info!(
      "Schema '{}' synced from MongoDB to {:?}",
      schema_id,
      self.local_path
    );
    Ok(schema)
  }

  /// Get schema from local JSON file (fast, offline-capable)
  /// Returns None if local file doesn't exist
  pub async fn get_local_schema(&self) -> OrmResult<Option<serde_json::Value>> {
    if !self.local_path.exists() {
      return Ok(None);
    }
    let content = std::fs::read_to_string(&self.local_path)?;
    let schema: serde_json::Value = serde_json::from_str(&content)?;
    Ok(Some(schema))
  }

  /// Get the local file path
  pub fn local_path(&self) -> &std::path::Path {
    &self.local_path
  }
}
