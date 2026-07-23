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
pub mod storage;
pub mod update;

pub use algorithms::{
  bubble_sort, bubble_sort_by, cap_string, escape_html, insertion_sort, insertion_sort_by,
  merge_sort, merge_sort_by, quick_sort, quick_sort_by, sanitize_for_mongo, sanitize_for_overlay,
  strip_urls, Algorithm, AlgorithmRegistry, EmailValidator, FieldValidator, FullTextSearch, Graph,
  GraphEdge, GraphNode, SearchAlgorithm, TextSearch, Validate, ValidationAlgorithm,
};
pub use commands::{
  check_for_update_command, delete_schema, download_update_command, execute_algorithm,
  get_all_schemas, get_current_version, get_schema, get_schema_direct, get_ui_schema,
  install_update_command, list_algorithms, save_schema, save_ui_schema,
};
pub use crud::service::CrudService;
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
  AppConfig, AppSettings, CanvasElement, ColorMode, CommandDef, ComponentDef, ComponentProp,
  DataBinding, EventSignature, GridArea, GridDefaults, GridElement, GridPosition, GridTemplate,
  GridTrack, I18nConfig, Layout, LayoutSlot, LocaleMap, ModuleDef, NamedGridArea, Page, PageMeta,
  PageSection, RenderedElement, RenderedPage, RenderedSection, ResponsiveBreakpoints,
  ResponsiveClasses, SchemaValidationError, ServiceCrud, ServiceDef, ServiceField, Theme,
  ThemeColors, UiSchema, ValidationResult,
};
pub use storage::{
  create_json_provider, create_json_provider_with_config, signal_store::SignalStore,
  JsonProviderState, SchemaSyncService,
};
pub use storage::{setup_schema_system, SchemaConfig, SchemaSyncState, SchemaSystem};
pub use update::{
  check_for_update, download_update, get_temp_download_path, install_update, CheckUpdateResult,
  DownloadProgress, GitHubAsset, GitHubRelease, Platform, UpdateInfo,
};

pub use nosql_orm::Entity;
