pub mod commands;
pub mod crud;
pub mod error;
pub mod extension;
pub mod logger;
pub mod rbac;
pub mod response;
pub mod result;
pub mod schema;
pub mod storage;
pub mod update;
pub mod validation;

pub use commands::{
  db_delete_schema, db_get_all_schemas, db_get_schema, db_save_schema,
  logger_commands::{clear_logs, get_log_entries, get_log_level, set_log_level, write_log_to_file},
  schema_commands::{get_ui_schema, save_ui_schema},
  sdui_commands::check_permission,
  update_commands::{check_for_update_command, download_update_command, install_update_command, get_current_version},
  KernelDb, KernelEntity,
};
pub use crud::{CrudFilter, CrudQuery, CrudResult, PaginatedResult};
pub use crud::service::CrudService;
pub use error::AppError;
pub use extension::{get_extension_names, init_extensions_with_app, DesignerExtension};
pub use logger::{FileLogger, LogEntry, LogLevel, Logger};
pub use rbac::{auth, commands as rbac_commands};
pub use response::{Response, Status};
pub use result::Result;
pub use schema::*;
pub use storage::{
  create_json_provider, create_json_provider_with_config, JsonProviderState, SignalStore,
};
pub use validation::*;
pub use update::{
    CheckUpdateResult, DownloadProgress, GitHubAsset, GitHubRelease, Platform, UpdateInfo,
};

pub use nosql_orm::Entity;

use std::sync::Arc;
use tauri::Manager;

/// Register CRUD-related state (CrudService) in the Tauri app.
/// Each app calls this during app setup to inject a shared CrudService for
/// schema-defined CRUD operations.
pub fn register_crud_commands(app: &mut tauri::App, provider: JsonProviderState) {
    let crud_service = Arc::new(CrudService::new(provider.as_ref().clone()));
    app.manage(crud_service);
}
