pub mod commands;
pub mod crud;
pub mod error;
pub mod extension;
pub mod http_client;
pub mod axum_handlers;
pub mod logger;
pub mod rbac;
pub mod response;
pub mod result;
pub mod schema;
pub mod websocket;
pub mod validation;
pub mod algorithms;
pub mod storage;
pub mod update;

pub use commands::*;
pub use error::AppError;
pub use http_client::{HttpClient, HttpResponse, AppError as HttpClientAppError};
pub use crud::service::CrudService;
pub use axum_handlers::*;
pub use logger::*;
pub use rbac::*;
pub use response::*;
pub use result::*;
pub use schema::*;
pub use storage::{create_json_provider, create_json_provider_with_config, JsonProviderState, SignalStore};
pub use update::{check_for_update, download_update, get_temp_download_path, install_update, CheckUpdateResult, Platform, UpdateInfo, DownloadProgress, GitHubAsset, GitHubRelease};
pub use algorithms::*;
pub use validation::*;

pub use nosql_orm::Entity;

use std::sync::Arc;
use tauri::Manager;

pub fn register_crud_commands(app: &mut tauri::App, provider: JsonProviderState) {
    let crud_service = Arc::new(CrudService::new(provider.as_ref().clone()));
    app.manage(crud_service);
}