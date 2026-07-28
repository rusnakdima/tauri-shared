pub mod algorithms;
pub mod commands;
pub mod crud;
pub mod env;
pub mod error;
pub mod extension;
pub mod logger;
pub mod rbac;
pub mod response;
pub mod result;
pub mod schema;
pub mod services;
pub mod storage;
pub mod update;

pub use algorithms::{
  quick_sort_by, sanitize_for_mongo, sanitize_for_overlay, Algorithm, AlgorithmRegistry, Graph,
  ValidationAlgorithm,
};
pub use commands::{
  algo_execute, check_for_update_command, crud_execute, delete_schema, download_update_command,
  get_all_schemas, get_current_version, get_schema, get_schema_direct, get_ui_schema,
  install_update_command, list_algorithms, save_schema, save_ui_schema,
};
pub use crud::service::CrudService;
pub use crud::{CrudResult, PaginatedResult};
pub use env::{init_env, EnvConfig, ENV};
pub use error::{AppError, ProjectError};
pub use logger::{FileLogger, LogEntry, LogLevel, Logger};
pub use rbac::{
  get_current_user, login, logout, rbac_assign_role_to_user, rbac_create_permission,
  rbac_create_role, rbac_delete_permission, rbac_delete_role, rbac_get_role_permissions,
  rbac_get_user_roles, rbac_grant_permission, rbac_list_permissions, rbac_list_roles,
  rbac_remove_role_from_user, rbac_revoke_permission, register, Permission, Role, RolePermission,
  Session, User, UserRole,
};
pub use response::{Response, Status};
pub use result::Result;
pub use schema::{
  ActionDef, AppConfig, AppSettings, CanvasElement, ColorMode, CommandDef, ComponentDef,
  ComponentProp, DataBinding, DataSourceDef, ElementLayout, EventSignature, GridArea, GridDefaults,
  GridElement, GridPosition, GridTemplate, GridTrack, HandlerDef, I18nConfig, Layout, LayoutSlot,
  LocaleMap, ModuleDef, NamedGridArea, Page, PageMeta, PageSection, RenderedElement, RenderedPage,
  RenderedSection, ResponsiveBreakpoints, ResponsiveClasses, SchemaValidationError, ServiceCrud,
  ServiceDef, ServiceField, StoreDef, Theme, ThemeColors, UiSchema, ValidationResult,
};
pub use services::BaseCrudService;
pub use storage::{
  create_json_provider, create_json_provider_with_config, signal_store::SignalStore,
  JsonProviderState, SchemaSyncService,
};
pub use storage::{setup_schema_system, SchemaConfig, SchemaSyncState, SchemaSystem};
pub use update::{
  check_for_update, download_update, get_temp_download_path, install_update, CheckUpdateResult,
  DownloadProgress, GitHubAsset, GitHubRelease, Platform, UpdateInfo,
};
