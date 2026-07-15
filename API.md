# tauri-shared — Rust API Reference

Auto-generated from source. For usage documentation, see [README.md](./README.md).

---

## Crate Root

```rust
pub mod commands;       // Tauri command handlers
pub mod crud;          // CRUD filter/query types
pub mod error;         // AppError type
pub mod extension;     // Designer extension system
pub mod http_client;   // HTTP client for cloud sync
pub mod logger;        // File logging
pub mod rbac;          // Role-based access control
pub mod response;      // Response<T> wrapper
pub mod result;        // Result type alias
pub mod schema;        // Schema types
pub mod storage;       // SignalStore, JsonProvider, SchemaSyncService
pub mod update;        // Auto-update system
pub mod validation;    // Input validation
pub mod websocket;     // WebSocket client
pub mod algorithms;    // Sorting, searching, graph algorithms
```

## Result & Response

### Result

```rust
pub use result::Result;  // = std::result::Result<T, AppError>
```

### Response

```rust
use tauri_shared::response::{Response, Status};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response<T> {
    pub status: Status,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Created,
    Updated,
    Deleted,
    Error,
    ValidationError,
    NotFound,
    Unauthorized,
    Forbidden,
}

impl<T> Response<T> {
    pub fn success(data: T, message: Option<&str>) -> Self
    pub fn created(data: T) -> Self
    pub fn updated(data: T) -> Self
    pub fn deleted(data: T) -> Self
    pub fn error(message: impl Into<String>) -> Self
    pub fn error_with_data(data: T, message: impl Into<String>) -> Self
    pub fn validation_error(message: impl Into<String>) -> Self
    pub fn not_found(message: impl Into<String>) -> Self
    pub fn unauthorized() -> Self
    pub fn forbidden() -> Self
}
```

## AppError

```rust
use tauri_shared::error::AppError;

pub enum AppError {
    NotFound(String),
    Validation(String),
    Unauthorized,
    Forbidden,
    Internal(String),
    Database(String),
    Io(String),
    Serialization(String),
}
```

Implements `From<T>` for common error types and `std::error::Error`.

## Schema Module

```rust
pub use schema::{
    App, Component, ComponentBehavior, DataBinding, EventBinding,
    GridPosition, GridTemplate, Layout, LayoutElement,
    ModuleDef, NavItem, Page, RenderedPage, ThemeConfig, UiSchema,
};
```

### UiSchema

```rust
pub struct UiSchema {
    pub pages: Vec<Page>,
    pub navigation: Vec<NavItem>,
    pub theme: Option<ThemeConfig>,
    pub globals: Option<serde_json::Value>,
}
```

### Page

```rust
pub struct Page {
    pub id: String,
    pub title: String,
    pub route: String,
    pub layout: Option<Layout>,
    pub elements: Vec<LayoutElement>,
    pub condition: Option<String>,
}
```

### LayoutElement

```rust
pub struct LayoutElement {
    pub id: String,
    pub component: Component,
    pub grid_position: Option<GridPosition>,
    pub children: Vec<LayoutElement>,
    pub region: Option<String>,
}
```

### Component

```rust
pub struct Component {
    pub component_type: String,
    pub properties: serde_json::Value,
    pub events: Vec<EventBinding>,
    pub behaviors: Vec<ComponentBehavior>,
}
```

### GridPosition

```rust
pub struct GridPosition {
    pub column: Option<String>,
    pub row: Option<String>,
    pub column_span: Option<i32>,
    pub row_span: Option<i32>,
}
```

### DataBinding

```rust
pub struct DataBinding {
    pub expression: String,
    pub context: serde_json::Value,
}
```

### RenderedPage

```rust
pub struct RenderedPage {
    pub page_id: String,
    pub route: String,
    pub html: String,
    pub state: serde_json::Value,
}
```

## Storage

```rust
pub use storage::{
    create_json_provider,
    create_json_provider_with_config,
    JsonProvider,
    JsonProviderState,
    SchemaSyncService,
    signal_store::SignalStore,
    setup_schema_system,
    SchemaConfig,
    SchemaSystem,
    SchemaSyncState,
};
```

### SignalStore

```rust
use tauri_shared::storage::signal_store::SignalStore;

impl SignalStore {
    pub fn new(provider: JsonProvider) -> Self
    pub async fn get(&self, key: &str) -> Result<serde_json::Value>
    pub async fn set(&self, key: &str, value: serde_json::Value) -> Result<()>
    pub async fn delete(&self, key: &str) -> Result<()>
    pub async fn keys(&self) -> Result<Vec<String>>
    pub async fn clear(&self) -> Result<()>
    pub fn on_change(&self) -> impl Stream<Item = (String, Option<serde_json::Value>)>
}

pub fn create_json_provider(path: &str) -> Result<JsonProvider>
pub fn create_json_provider_with_config(path: &str, max_size: usize) -> Result<JsonProvider>
```

## RBAC

```rust
pub use rbac::{
    auth::{login, logout, refresh_token},
    permissions::{Permission, PermissionContext},
    roles::{Role, RoleBinding},
    AuthContext, User,
};

impl Permission {
    pub fn matches(&self, resource: &str, action: &str) -> bool
}

impl User {
    pub fn has_permission(&self, permission: &Permission) -> bool
    pub fn has_role(&self, role_name: &str) -> bool
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<Role>,
}

pub struct AuthContext {
    pub user: User,
    pub token: String,
}
```

## Logger

```rust
pub use logger::{
    FileLogger,
    LogEntry,
    LogLevel,
    Logger,
};

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str
}

impl LogEntry {
    pub fn new(level: LogLevel, message: &str) -> Self
}

impl FileLogger {
    pub fn new(path: &str) -> Result<Self>
    pub async fn write(&self, level: LogLevel, message: &str) -> Result<()>
    pub async fn read(&self, limit: usize) -> Result<Vec<LogEntry>>
    pub async fn clear(&self) -> Result<()>
}
```

## CRUD

```rust
pub use crud::{
    service::CrudService,
    CrudFilter, CrudQuery, CrudResult, PaginatedResult,
};

pub struct CrudFilter {
    pub field: String,
    pub op: String,
    pub value: serde_json::Value,
}

pub struct CrudQuery {
    pub filters: Vec<CrudFilter>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub struct CrudResult<T> {
    pub data: T,
    pub total: usize,
}

pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub total_count: usize,
}
```

## Validation

```rust
pub use validation::{validate_name, validate_sql, validate_connection_id};

pub fn validate_name(name: &str) -> Result<()>
pub fn validate_sql(identifier: &str) -> Result<()>
pub fn validate_connection_id(id: &str) -> Result<()>
```

## Kernel Entity

```rust
pub use commands::KernelEntity;

pub trait KernelEntity: Send + Sync {
    const ENTITY_NAME: &'static str;
    fn entity_name(&self) -> &str { Self::ENTITY_NAME }
}
```

## Extension System

```rust
pub use extension::{DesignerExtension, designer_extension, init_extensions_with_app, get_extension_names};

pub trait DesignerExtension: Send + Sync {
    fn name(&self) -> &str;
    fn register_commands(&self, app: &mut tauri::App) {}
    fn init(&self, app: &AppHandle) {}
}

pub fn init_extensions_with_app(app: &mut tauri::App)
pub fn get_extension_names() -> Vec<String>
pub fn register_extension<E: DesignerExtension + 'static>(extension: E)

// Macro: designer_extension!(MyExtension);
```

Allows `@Designer/` libraries to extend the IPC command API without modifying the main Designer application.

## HTTP Client

```rust
pub use http_client::{HttpClient, HttpResponse, AppError as HttpClientAppError};

pub struct HttpClient {
    pub base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self
    pub async fn get<T>(&self, path: String) -> Result<HttpResponse<T>>
    pub async fn post<T>(&self, path: String, body: Value) -> Result<HttpResponse<T>>
    pub async fn put<T>(&self, path: String, body: Value) -> Result<HttpResponse<T>>
    pub async fn delete<T>(&self, path: String) -> Result<HttpResponse<T>>
}

pub struct HttpResponse<T> {
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
    pub timestamp: i64,
}
```

## WebSocket Client

```rust
pub use websocket::WsClient;

pub struct WsClient {
    pub url: String,
}

impl WsClient {
    pub fn new(url: String) -> Self
    pub async fn send(&self, message: String) -> Result<(), AppError>
    pub async fn receive(&self) -> Result<String, AppError>
    pub async fn subscribe(&self, channel: String) -> Result<String, AppError>
}
```

## Auto-Update

```rust
pub use update::{
    check_for_update, download_update, get_temp_download_path, install_update,
    CheckUpdateResult, Platform, UpdateInfo, DownloadProgress, GitHubAsset, GitHubRelease,
};

pub struct CheckUpdateResult {
    pub has_update: bool,
    pub update_info: Option<UpdateInfo>,
    pub error: Option<String>,
}

pub fn check_for_update() -> Result<CheckUpdateResult>
pub async fn download_update() -> Result<DownloadProgress>
pub fn get_temp_download_path() -> String
pub fn install_update() -> Result<()>

pub enum Platform { Mac, MacArch, Windows, Linux, Android, Ios }
```

## Algorithm Commands (feature = "algorithms")

```rust
pub use algorithms::{ValidationAlgorithm, SearchAlgorithm};

impl ValidationAlgorithm {
    pub fn validate_input(input: &str, max_length: usize) -> bool
    pub fn validate_email(email: &str) -> bool
    pub fn sanitize_input(input: &str) -> String
}

impl SearchAlgorithm {
    pub fn search_schemas<T: AsRef<str> + Clone>(items: &[T], query: &str) -> Vec<T>
    pub fn paginate<T>(items: &[T], page: u64, limit: u64) -> Vec<T>
}
```

## Tauri Commands

All commands use `#[tauri::command]` and return `Result<T>`:

```rust
// Schema DB (nosql_orm JsonProvider)
#[tauri::command] pub async fn db_get_schema(db: State<'_, JsonProvider>, id: String) -> Result<UiSchema>
#[tauri::command] pub async fn db_save_schema(db: State<'_, JsonProvider>, id: String, schema: UiSchema) -> Result<()>
#[tauri::command] pub async fn db_get_all_schemas(db: State<'_, JsonProvider>) -> Result<Vec<UiSchema>>
#[tauri::command] pub async fn db_delete_schema(db: State<'_, JsonProvider>, id: String) -> Result<()>

// Schema commands (alternative API)
#[tauri::command] pub async fn get_schema(db: State<'_, JsonProvider>, id: String) -> Result<UiSchema>
#[tauri::command] pub async fn save_schema(db: State<'_, JsonProvider>, schema: UiSchema) -> Result<()>
#[tauri::command] pub async fn get_ui_schema(db: State<'_, JsonProvider>, id: String) -> Result<Response<serde_json::Value>>
#[tauri::command] pub async fn save_ui_schema(db: State<'_, JsonProvider>, id: String, schema: serde_json::Value) -> Result<Response<()>>
#[tauri::command] pub async fn delete_schema(db: State<'_, JsonProvider>, id: String) -> Result<()>

// Schema sync (MongoDB cloud → local JSON)
#[tauri::command] pub async fn get_schema_local_first(id: String, state: State<'_, Arc<SchemaSyncState>>) -> Result<Response<serde_json::Value>>
#[tauri::command] pub async fn sync_schema_from_cloud(id: String, state: State<'_, Arc<SchemaSyncState>>) -> Result<Response<serde_json::Value>>

// RBAC
#[tauri::command] pub fn check_permission(permission: Permission, resource: String, action: String) -> Result<bool>

// Logger
#[tauri::command] pub async fn get_log_entries(limit: usize) -> Result<Vec<LogEntry>>
#[tauri::command] pub async fn get_log_level() -> Result<LogLevel>
#[tauri::command] pub async fn set_log_level(level: LogLevel) -> Result<()>
#[tauri::command] pub async fn clear_logs() -> Result<()>
#[tauri::command] pub async fn write_log_to_file() -> Result<()>

// Kernel DB
#[tauri::command] pub async fn kernel_find_by_id(id: String) -> Result<Option<serde_json::Value>>
#[tauri::command] pub async fn kernel_find_all(query: CrudQuery) -> Result<CrudResult<Vec<serde_json::Value>>>
#[tauri::command] pub async fn kernel_create(data: serde_json::Value) -> Result<serde_json::Value>
#[tauri::command] pub async fn kernel_update(id: String, data: serde_json::Value) -> Result<serde_json::Value>
#[tauri::command] pub async fn kernel_delete(id: String) -> Result<()>

// Update
#[tauri::command] pub async fn check_for_update_command() -> Result<CheckUpdateResult>
#[tauri::command] pub async fn download_update_command() -> Result<DownloadProgress>
#[tauri::command] pub async fn install_update_command() -> Result<()>
#[tauri::command] pub fn get_current_version() -> String
```
