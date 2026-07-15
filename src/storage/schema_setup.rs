//! SchemaSetup - single initialization function for the schema system.
//!
//! Apps call `setup_schema_system()` with just config values.
//! Library handles: JsonProvider creation, SchemaSyncService, MongoDB sync, state management.

use crate::log_info;
use crate::storage::json_provider::JsonProviderState;
use crate::storage::schema_sync_service::SchemaSyncService;
use crate::storage::signal_store::SignalStore;
use nosql_orm::prelude::*;
use nosql_orm::provider::DatabaseProvider;
use std::path::Path;
use std::sync::Arc;

/// Configuration for schema system initialization.
/// Apps pass this struct — library handles everything.
pub struct SchemaConfig {
  /// App identifier (e.g., "translator", "cleanux")
  pub app_id: String,
  /// App data directory (from Tauri's `app.path().app_data_dir()`)
  pub data_dir: std::path::PathBuf,
  /// MongoDB connection URI (from env: MONGO_URI)
  pub mongo_uri: String,
  /// MongoDB database name for schemas (from env: SCHEMA_DB_NAME, default: "schemas")
  pub schema_db_name: String,
}

impl SchemaConfig {
  /// Create config from environment variables.
  /// Falls back to sensible defaults if env vars are not set.
  pub fn from_env(app_id: &str, data_dir: std::path::PathBuf) -> Self {
    Self {
      app_id: app_id.to_string(),
      data_dir,
      mongo_uri: std::env::var("MONGO_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
      schema_db_name: std::env::var("SCHEMA_DB_NAME").unwrap_or_else(|_| "schemas".to_string()),
    }
  }
}

/// Result of schema system initialization.
/// Apps receive this and manage the state.
pub struct SchemaSystem {
  /// JsonProvider for all data access (schemas + app data)
  pub db: JsonProviderState,
  /// SchemaSyncService for cloud-to-local sync
  pub sync_service: Option<Arc<SchemaSyncState>>,
}

/// State wrapper for SchemaSyncService (Arc-wrapped for Tauri State)
pub struct SchemaSyncState {
  pub service: SchemaSyncService,
}

impl SchemaSyncState {
  pub fn new(service: SchemaSyncService) -> Self {
    Self { service }
  }
}

/// Setup the entire schema system with a single call.
///
/// This function:
/// 1. Creates JsonProvider (local JSON database)
/// 2. Creates SchemaSyncService (MongoDB → local sync)
/// 3. Syncs schema from MongoDB on startup
/// 4. Inserts synced schema into JsonProvider
/// 5. Returns managed state for Tauri
///
/// # Example (in app's lib.rs)
/// ```rust
/// use tauri_shared::storage::setup_schema_system;
/// use tauri_shared::storage::SchemaConfig;
///
/// let config = SchemaConfig::from_env("translator", data_dir);
/// let system = setup_schema_system(config).await?;
///
/// app.manage(system.db);
/// if let Some(sync) = system.sync_service {
///     app.manage(sync);
/// }
/// ```
pub async fn setup_schema_system(
  config: SchemaConfig,
) -> Result<SchemaSystem, Box<dyn std::error::Error>> {
  log_info!("Setting up schema system for app '{}'", config.app_id);

  // 1. Create JsonProvider (local JSON database)
  let db = crate::storage::create_json_provider(&config.data_dir)
    .await
    .map_err(|e| format!("Failed to create JsonProvider: {}", e))?;
  let db: JsonProviderState = Arc::new(db);

  // 2. Create SchemaSyncService
  let schema_path = config.data_dir.join("schemas.json");
  let sync_service =
    match SchemaSyncService::new(schema_path, &config.mongo_uri, &config.schema_db_name).await {
      Ok(service) => {
        log_info!("SchemaSyncService created successfully");
        Some(Arc::new(SchemaSyncState::new(service)))
      }
      Err(e) => {
        log_info!("SchemaSyncService creation failed (non-fatal): {}", e);
        None
      }
    };

  // 3. Sync schema from MongoDB on startup
  if let Some(ref sync_state) = sync_service {
    match sync_state.service.sync_schema(&config.app_id).await {
      Ok(synced_schema) => {
        log_info!("Schema '{}' synced from MongoDB on startup", config.app_id);

        // 4. Insert synced schema into JsonProvider
        let mut doc = synced_schema;
        if doc
          .get("id")
          .and_then(|v| v.as_str())
          .is_none_or(|s| s.is_empty())
        {
          doc["id"] = serde_json::json!(config.app_id);
        }

        let db_clone = db.clone();
        let app_id = config.app_id.clone();
        if let Err(e) = tokio::spawn(async move {
          if db_clone.find_by_id("schemas", &app_id).await?.is_some() {
            db_clone.update("schemas", &app_id, doc).await?;
            log_info!("Updated schema in JsonProvider");
          } else {
            db_clone.insert("schemas", doc).await?;
            log_info!("Inserted schema into JsonProvider");
          }
          Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await
        {
          log_info!("Failed to insert schema into JsonProvider: {}", e);
        }
      }
      Err(e) => {
        log_info!(
          "Failed to sync schema '{}' on startup (non-fatal): {}",
          config.app_id,
          e
        );
      }
    }
  }

  log_info!("Schema system setup complete for app '{}'", config.app_id);

  Ok(SchemaSystem { db, sync_service })
}
