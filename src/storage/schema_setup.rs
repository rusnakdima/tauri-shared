//! SchemaSetup - single initialization function for the schema system.
//!
//! Apps call `setup_schema_system()` with just config values.
//! Library handles: JsonProvider creation, SchemaSyncService, MongoDB sync,
//! central /schemas/ directory dev fallback, state management.
//!
//! # Architecture
//!
//! The schema system always reads from JsonProvider at the app's local data dir.
//! - PROD: MongoDB sync writes into JsonProvider → flushed to {data_dir}/schemas.json
//! - DEV:  /schemas/{app_id}schemas.json is copied to {data_dir}/schemas.json
//!         as a JsonProvider-compatible array before JsonProvider reads it.

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

/// Write a schema JSON file as a JsonProvider-compatible array.
/// Reads a single object from `source` and writes it as `[{...}]` to `dest`.
fn write_schema_as_json_provider_array(source: &std::path::Path, dest: &std::path::Path) -> bool {
  let content = match std::fs::read_to_string(source) {
    Ok(c) => c,
    Err(e) => {
      log_info!("Failed to read central schema file: {}", e);
      return false;
    }
  };

  let value: serde_json::Value = match serde_json::from_str(&content) {
    Ok(v) => v,
    Err(e) => {
      log_info!("Failed to parse central schema file: {}", e);
      return false;
    }
  };

  let array = match value {
    serde_json::Value::Array(arr) => arr,
    serde_json::Value::Object(_) => vec![value],
    _ => {
      log_info!("Central schema file has unexpected format");
      return false;
    }
  };

  if let Some(parent) = dest.parent() {
    let _ = std::fs::create_dir_all(parent);
  }

  match serde_json::to_string_pretty(&array) {
    Ok(json) => match std::fs::write(dest, &json) {
      Ok(_) => {
        log_info!(
          "Schema written to {:?} as JsonProvider array ({} documents)",
          dest,
          array.len()
        );
        true
      }
      Err(e) => {
        log_info!("Failed to write schema to {:?}: {}", dest, e);
        false
      }
    },
    Err(e) => {
      log_info!("Failed to serialize schema array: {}", e);
      false
    }
  }
}

/// Copy the central dev schema to the app data dir in JsonProvider array format.
/// This creates the file-level "link" so JsonProvider reads it naturally from disk.
fn link_dev_schema_to_data_dir(app_id: &str, data_dir: &std::path::Path) {
  let schemas_dir =
    std::env::var("SCHEMAS_DIR").unwrap_or_else(|_| "/home/dmitriy/Projects/schemas/".to_string());
  let source = std::path::Path::new(&schemas_dir).join(format!("{}schemas.json", app_id));

  if !source.exists() {
    log_info!("Dev schema not found at {:?} — no file linked", source);
    return;
  }

  let dest = data_dir.join("schemas.json");
  log_info!("Linking dev schema: {:?} → {:?}", source, dest);

  write_schema_as_json_provider_array(&source, &dest);
}

/// Setup the entire schema system with a single call.
///
/// This function:
/// 1. Creates JsonProvider (local JSON database at {data_dir}/)
/// 2. Creates SchemaSyncService (MongoDB → local sync)
/// 3. Tries MongoDB sync — if success, writes into JsonProvider → flushed to file
/// 4. If MongoDB fails, copies central /schemas/ file to {data_dir}/schemas.json
///    so JsonProvider reads it from disk (dev mode "linking")
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

  // 4. Dev fallback: copy central schema file to {data_dir}/schemas.json
  //    This creates the file-level "link" so JsonProvider reads it from disk
  if !schema_loaded {
    log_info!("Dev mode: linking schema from /schemas/ to app data dir");
    link_dev_schema_to_data_dir(&config.app_id, &config.data_dir);
  }

  log_info!("Schema system setup complete for app '{}'", config.app_id);

  Ok(SchemaSystem { db, sync_service })
}
