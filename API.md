# tauri-shared — Rust API Reference

Auto-generated from source. For usage documentation, see [README.md](./README.md).

---

## Crate Root

```rust
pub mod commands;       // Tauri command handlers
pub mod crud;          // CRUD filter/query types
pub mod error;         // AppError type
pub mod i18n;          // Translation
pub mod logger;         // File logging
pub mod lru;           // LRU cache
pub mod macros;        // Procedural macros
pub mod migration;      // DB migration system
pub mod rbac;          // Role-based access control
pub mod repository;     // Kernel repository
pub mod response;      // Response<T> wrapper
pub mod result;        // Result type alias
pub mod runtime;       // SDUI engine
pub mod schema;        // Schema types
pub mod storage;       // SignalStore, JsonProvider
pub mod sync;          // Sync engine
pub mod typescript;    // TypeScript bindings generator
pub mod validation;    // Input validation

#[cfg(feature = "algorithms")]
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

## Runtime / SDUI Engine

```rust
pub use runtime::{SduiEngine, Router};

impl SduiEngine {
    pub fn from(schema: UiSchema) -> Self
    pub fn load_schema(&mut self, schema: UiSchema)
    pub fn render_page(&self, route: &str) -> Result<RenderedPage>
    pub fn resolve_binding(&self, binding: &DataBinding) -> Result<serde_json::Value>
    pub fn navigate(&mut self, route: &str) -> Result<()>
}

impl Router {
    pub fn resolve_route(&self, path: &str) -> Option<String>
}
```

## Storage

```rust
pub use storage::{
    create_json_provider,
    create_json_provider_with_config,
    JsonProvider,
    JsonProviderState,
    SignalStore,
};
```

### SignalStore

```rust
use tauri_shared::storage::SignalStore;

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

## Sync Engine

```rust
pub use sync::{
    MongoBridge,
    SchemaSyncService,
    SyncEngine,
    SyncOperation,
    SyncQueue,
};

pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

pub struct SyncQueue {
    pub operations: Vec<SyncOperation>,
}

impl SyncEngine {
    pub fn new(bridge: MongoBridge, queue: SyncQueue) -> Self
    pub async fn sync_to_cloud(&self) -> Result<()>
    pub async fn pull_from_cloud(&self) -> Result<UiSchema>
}

impl MongoBridge {
    pub fn new(uri: &str, db_name: &str) -> Self
    pub async fn push(&self, schema: &UiSchema) -> Result<()>
    pub async fn pull(&self) -> Result<UiSchema>
}
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

## Migration

```rust
pub use migration::{Migration, MigrationError};

#[async_trait::async_trait]
pub trait Migration: Send + Sync {
    fn id(&self) -> &str;
    fn description(&self) -> &str;
    async fn up(&self, db: &Database) -> Result<(), MigrationError>;
    async fn down(&self, db: &Database) -> Result<(), MigrationError>;
}

pub enum MigrationError {
    Internal(String),
    RollbackFailed(String),
}
```

## CRUD

```rust
pub use crud::{CrudFilter, CrudQuery, CrudResult, PaginatedResult};

pub enum CrudFilter {
    Eq(String, serde_json::Value),
    Ne(String, serde_json::Value),
    Gt(String, serde_json::Value),
    Lt(String, serde_json::Value),
    Contains(String, serde_json::Value),
    In(String, Vec<serde_json::Value>),
}

pub struct CrudQuery {
    pub filters: Vec<CrudFilter>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

pub struct CrudResult<T> {
    pub data: T,
    pub total: usize,
}

pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub page: usize,
    pub page_size: usize,
    pub total: usize,
}
```

## Repository

```rust
pub use repository::KernelRepository;

pub trait KernelRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<serde_json::Value>>;
    async fn find_all(&self, query: CrudQuery) -> Result<CrudResult<Vec<serde_json::Value>>>;
    async fn create(&self, data: serde_json::Value) -> Result<serde_json::Value>;
    async fn update(&self, id: &str, data: serde_json::Value) -> Result<serde_json::Value>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn count(&self) -> Result<usize>;
}
```

## LRU Cache

```rust
pub use lru::LruCache;

impl<K: Eq + Hash + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self
    pub fn get(&self, key: &K) -> Option<V>
    pub fn put(&mut self, key: K, value: V) -> Option<V>
    pub fn remove(&mut self, key: &K) -> Option<V>
    pub fn clear(&mut self)
    pub fn len(&self) -> usize
    pub fn is_empty(&self) -> bool
}
```

## Validation

```rust
pub use validation::{validate_name, validate_sql, validate_connection_id};

pub fn validate_name(name: &str) -> Result<()>
pub fn validate_sql(identifier: &str) -> Result<()>
pub fn validate_connection_id(id: &str) -> Result<()>
```

## i18n

```rust
pub use i18n::{translate, tauri_translate};

pub fn translate(
    key: &str,
    args: &[(&str, &str)],
    locale: &str,
) -> Result<String>;

#[cfg(feature = "tauri")]
pub async fn tauri_translate(
    key: &str,
    args: &[(&str, &str)],
) -> Result<String>;
```

## TypeScript Bindings

```rust
pub use typescript::{generate_typescript_bindings, schema_ts_bindings, ts_inline, ToTypeScript};

pub trait ToTypeScript {
    fn to_typescript(&self) -> String;
}

pub fn generate_typescript_bindings<T: ToTypeScript>() -> String
pub fn schema_ts_bindings() -> String
pub fn ts_inline<T: ToTypeScript>() -> String
```

## Kernel Entity

```rust
pub use commands::KernelEntity;

pub trait KernelEntity: Send + Sync {
    const ENTITY_NAME: &'static str;
    fn entity_name(&self) -> &str { Self::ENTITY_NAME }
}
```

## Macros

```rust
pub use macros::impl_entity_commands_inner;

impl_entity_commands_inner!(MyEntity, "entity_name");
```

Generates: `entity_create_<name>`, `entity_read_<name>`, `entity_update_<name>`, `entity_delete_<name>`, `entity_list_<name>`, `entity_count_<name>`.

## Algorithm Commands (feature = "algorithms")

Requires `features = ["algorithms"]` in Cargo.toml.

```rust
pub use algorithms::sorting::{bubble_sort, insertion_sort, merge_sort, quick_sort};
pub use algorithms::searching::{binary_search, linear_search};
pub use algorithms::graph::{dijkstra, bfs, dfs};
```

All algorithms operate on `Vec<T: Ord>` or graphs defined with `petgraph`.

## Tauri Commands

All commands use `#[tauri::command]` and return `Result<T>`:

```rust
// Schema DB
#[tauri::command] pub async fn db_get_schema(id: String) -> Result<UiSchema>
#[tauri::command] pub async fn db_save_schema(id: String, schema: UiSchema) -> Result<()>
#[tauri::command] pub async fn db_get_all_schemas() -> Result<Vec<UiSchema>>
#[tauri::command] pub async fn db_delete_schema(id: String) -> Result<()>

// SDUI
#[tauri::command] pub async fn load_schema(schema: UiSchema, state: State<'_, SduiEngineState>) -> Result<UiSchema>
#[tauri::command] pub fn render_page(route: String, state: State<'_, SduiEngineState>) -> Result<RenderedPage>
#[tauri::command] pub fn resolve_binding(binding: DataBinding, state: State<'_, SduiEngineState>) -> Result<serde_json::Value>
#[tauri::command] pub async fn sync_to_cloud(state: State<'_, Arc<SyncEngine>>) -> Result<()>
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
```
