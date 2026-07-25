//! SchemaSetup - single initialization function for the schema system.
//!
//! Apps call `setup_schema_system()` with just config values.
//! Library handles: JsonProvider creation, SchemaSyncService, MongoDB sync,
//! state management.
//!
//! # Architecture
//!
//! The schema system reads from {data_dir}/schemas.json via JsonProvider.
//! A symlink at {data_dir}/schemas.json → /home/dmitriy/Projects/schemas/{app_id}schemas.json
//! is created MANUALLY by the user (not by this library).
//!
//! - PROD: MongoDB sync writes into JsonProvider → flushed to {data_dir}/schemas.json
//! - DEV:  {data_dir}/schemas.json is a symlink to the canonical schema file

use crate::log_info;
use crate::storage::json_provider::JsonProviderState;
use crate::storage::schema_sync_service::SchemaSyncService;
use std::sync::Arc;

/// Configuration for schema system initialization.
/// Apps pass this struct — library handles everything.
#[derive(Clone)]
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
/// 1. Creates JsonProvider (local JSON database at {data_dir}/)
/// 2. Creates SchemaSyncService (MongoDB → local sync)
/// 3. Tries MongoDB sync — if success, writes into JsonProvider → flushed to file
/// 4. If MongoDB fails, the app reads from {data_dir}/schemas.json (which is a
///    symlink to /home/dmitriy/Projects/schemas/{app_id}schemas.json — created manually)
/// 5. Returns managed state for Tauri
///
/// Always reads from local app data dir via JsonProvider.
///
/// # Example (in app's lib.rs)
/// ```ignore
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

  // 1. Create SchemaSyncService (MongoDB connection)
  let mongo_available = std::env::var("MONGO_URI").is_ok();
  let sync_service = if mongo_available {
    match SchemaSyncService::new(&config.mongo_uri, &config.schema_db_name).await {
      Ok(service) => {
        log_info!("SchemaSyncService created successfully (MongoDB available)");
        Some(Arc::new(SchemaSyncState::new(service)))
      }
      Err(e) => {
        log_info!("MongoDB connection failed (non-fatal): {}", e);
        None
      }
    }
  } else {
    log_info!("MONGO_URI not set — dev mode, no MongoDB sync");
    None
  };

  // 2. Create JsonProvider (local JSON database at {data_dir}/)
  let db = crate::storage::create_json_provider(&config.data_dir)
    .await
    .map_err(|e| format!("Failed to create JsonProvider: {}", e))?;
  let db: JsonProviderState = Arc::new(db);

  // 3. Try MongoDB sync → writes into JsonProvider → flushed to {data_dir}/schemas.json
  let mut schema_loaded = false;
  if let Some(ref sync_state) = sync_service {
    match sync_state.service.sync_schema(&config.app_id, &db).await {
      Ok(_) => {
        log_info!("Schema '{}' synced from MongoDB on startup", config.app_id);
        schema_loaded = true;
      }
      Err(e) => {
        log_info!(
          "Failed to sync schema '{}' from MongoDB (non-fatal): {}",
          config.app_id,
          e
        );
      }
    }
  }

  // 4. If MongoDB sync failed, the app reads from {data_dir}/schemas.json directly.
  //    The user must have created a symlink: {data_dir}/schemas.json →
  //    /home/dmitriy/Projects/schemas/{app_id}schemas.json
  if !schema_loaded {
    log_info!(
      "Schema '{}' not loaded from MongoDB — ensure {}/schemas.json is a symlink to the canonical schema",
      config.app_id,
      config.data_dir.display()
    );
  }

  log_info!("Schema system setup complete for app '{}'", config.app_id);

  Ok(SchemaSystem { db, sync_service })
}
