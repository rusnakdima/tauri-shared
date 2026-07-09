# tauri-shared

Rust backend library for Tauri desktop applications. Provides schema management, SDUI rendering pipeline, sync engine, storage, RBAC, logging, migration system, and algorithm implementations.

## Adding to Your Project

```toml
# Cargo.toml
[dependencies]
tauri-shared = { path = "../tauri-shared" }
```

Or from git:
```toml
[dependencies]
tauri-shared = { git = "https://github.com/your-org/tauri-shared", branch = "main" }
```

**Feature flags:**
```toml
[dependencies]
tauri-shared = { path = "../tauri-shared", features = ["algorithms"] }
```

## Core Commands (Tauri IPC)

All commands are `async` and callable from the frontend via `@tauri-apps/api`:

### Schema Commands

```rust
// In your Tauri app's lib.rs:
use tauri_shared::commands::{db_get_schema, db_save_schema, db_get_all_schemas, db_delete_schema};
```

| Command | Signature | Description |
|---|---|---|
| `db_get_schema` | `(id: String) -> Result<UiSchema>` | Fetch schema by ID from nosql_orm |
| `db_save_schema` | `(id: String, schema: UiSchema) -> Result<()>` | Save schema to nosql_orm |
| `db_get_all_schemas` | `() -> Result<Vec<UiSchema>>` | List all schemas |
| `db_delete_schema` | `(id: String) -> Result<()>` | Delete schema by ID |

### SDUI Commands

```rust
use tauri_shared::commands::sdui_commands::{load_schema, render_page, resolve_binding, sync_to_cloud, check_permission};
```

| Command | Signature | Description |
|---|---|---|
| `load_schema` | `(schema: UiSchema) -> Result<UiSchema>` | Load schema into engine, returns it |
| `render_page` | `(route: String) -> Result<RenderedPage>` | Render a page by route |
| `resolve_binding` | `(binding: DataBinding) -> Result<serde_json::Value>` | Resolve a data binding expression |
| `sync_to_cloud` | `() -> Result<()>` | Sync current schema to cloud |
| `check_permission` | `(permission: Permission, resource: String, action: String) -> Result<bool>` | Check if permission matches resource/action |

### Logger Commands

```rust
use tauri_shared::commands::logger_commands::{get_log_entries, get_log_level, set_log_level, clear_logs, write_log_to_file};
```

### Algorithm Commands

Requires `features = ["algorithms"]`:

```rust
use tauri_shared::commands::algorithm_commands::{bubble_sort, merge_sort, quick_sort, insertion_sort, dijkstra};
```

## Schema System

The schema system is the backbone of SDUI (Schema-Driven UI):

### Core Types

```rust
use tauri_shared::schema::{UiSchema, Page, Layout, LayoutElement, Component, DataBinding};

pub struct UiSchema {
    pub pages: Vec<Page>,
    pub navigation: Vec<NavItem>,
    pub theme: Option<ThemeConfig>,
}

pub struct Page {
    pub id: String,
    pub title: String,
    pub route: String,
    pub layout: Option<Layout>,
    pub elements: Vec<LayoutElement>,
}

pub struct LayoutElement {
    pub id: String,
    pub component: Component,
    pub grid_position: Option<GridPosition>,
    pub children: Vec<LayoutElement>,
    pub region: Option<String>,
}

pub struct Component {
    pub component_type: String,  // e.g. "app-button"
    pub properties: serde_json::Value,
    pub events: Vec<EventBinding>,
    pub behaviors: Vec<ComponentBehavior>,
}
```

### SDUI Engine

```rust
use tauri_shared::runtime::SduiEngine;

let engine = SduiEngine::from(schema);
let rendered = engine.render_page("/home")?;
let value = engine.resolve_binding(&binding)?;
```

### Grid System

```rust
use tauri_shared::schema::{GridPosition, GridTemplate};

let pos = GridPosition {
    column: Some(1),
    row: Some(1),
    column_span: Some(2),
    row_span: Some(1),
};
```

## Storage

### SignalStore

Signal-based reactive key/value storage:

```rust
use tauri_shared::storage::{SignalStore, create_json_provider, JsonProviderState};

let store = SignalStore::new(provider);
store.set("key", serde_json::json!({"foo": "bar"})).await?;
let value: serde_json::Value = store.get("key").await?;
```

### JsonProvider

```rust
use tauri_shared::storage::{create_json_provider, create_json_provider_with_config};

let provider = create_json_provider()?;
let provider = create_json_provider_with_config(path, max_size)?;
```

## Sync Engine

```rust
use tauri_shared::sync::{SyncEngine, SyncQueue, SyncOperation, MongoBridge};

pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

pub struct SyncQueue {
    pub operations: Vec<SyncOperation>,
}

// MongoBridge for cloud sync
let bridge = MongoBridge::new(uri, db_name);
let sync_engine = SyncEngine::new(bridge, queue);
sync_engine.sync_to_cloud().await?;
```

## RBAC

```rust
use tauri_shared::rbac::{Permission, Role, User, AuthContext};

let permission = Permission {
    id: "post:delete".into(),
    name: "Delete Post".into(),
    resource: "post".into(),
    action: "delete".into(),
};

let matches = permission.matches(&"post", &"delete"); // true

// Check role permissions
user.has_permission(&permission); // bool
```

### Auth

```rust
use tauri_shared::rbac::auth::{login, logout, refresh_token};

let token = login(username, password).await?;
logout(token).await?;
```

## Response Type

All commands return `Result<T>` which is `AppError`-based:

```rust
use tauri_shared::{Response, Status, Result};

#[derive(serde::Serialize)]
pub struct Response<T> {
    pub status: Status,
    pub message: String,
    pub data: Option<T>,
}

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

## Logging

```rust
use tauri_shared::logger::{Logger, FileLogger, LogLevel, LogEntry};

let logger = FileLogger::new(path)?;
logger.write(LogLevel::Info, "Application started")?;
logger.write(LogLevel::Error, "Something failed")?;

let entries = logger.read(10).await?;
```

### Log Levels

```rust
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
```

## Migration System

```rust
use tauri_shared::migration::{Migration, MigrationError};

pub struct MyMigration;

#[async_trait::async_trait]
impl Migration for MyMigration {
    fn id(&self) -> &str { "001" }
    fn description(&self) -> &str { "Create users table" }

    async fn up(&self, db: &Database) -> Result<(), MigrationError> {
        db.execute("CREATE TABLE users ...").await?;
        Ok(())
    }

    async fn down(&self, db: &Database) -> Result<(), MigrationError> {
        db.execute("DROP TABLE users").await?;
        Ok(())
    }
}
```

## LRU Cache

```rust
use tauri_shared::lru::LruCache;

let mut cache = LruCache::new(capacity);
cache.put("key", "value");
let value = cache.get("key");
```

## Algorithms (feature = "algorithms")

Requires `features = ["algorithms"]` in Cargo.toml.

```rust
use tauri_shared::algorithms::{sort, search, graph};

let sorted = merge_sort(&[3, 1, 4, 1, 5]);
let found = binary_search(&sorted, &4);
let path = dijkstra(&graph, start, end);
```

### Sorting
- `bubble_sort`
- `insertion_sort`
- `merge_sort`
- `quick_sort`

### Searching
- `binary_search`
- `linear_search`

### Graph
- `dijkstra`
- `bfs`
- `dfs`

## TypeScript Bindings Generator

Generate TypeScript interfaces from Rust structs:

```rust
use tauri_shared::typescript::{generate_typescript_bindings, ToTypeScript};

#[derive(ToTypeScript)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

let ts_code = generate_typescript_bindings::<User>();
// Output: "export interface User { id: string; name: string; email: string; }"
```

## i18n

```rust
use tauri_shared::i18n::translate;

let translated = translate(
    "hello",
    &[("name", "Alice")],
    &locale,
)?;
```

## Validation

```rust
use tauri_shared::validation::{validate_name, validate_sql, validate_connection_id};

// Validate names (alphanumeric + underscores)
validate_name("user_123")?;

// Validate SQL identifiers
validate_sql("table_name")?;

// Validate connection IDs
validate_connection_id("conn_abc123")?;
```

## Error Handling

```rust
use tauri_shared::AppError;

match result {
    Ok(value) => value,
    Err(AppError::NotFound(msg)) => handle_not_found(msg),
    Err(AppError::Validation(msg)) => handle_validation(msg),
    Err(AppError::Unauthorized) => handle_unauthorized(),
    Err(AppError::Internal(msg)) => handle_internal(msg),
    Err(e) => handle_other(e),
}
```

## Entity Commands (CRUD)

Use the `impl_entity_commands_inner!` macro to generate CRUD commands for an entity:

```rust
use tauri_shared::macros::impl_entity_commands_inner;
use tauri_shared::crud::{CrudFilter, CrudQuery};

impl_entity_commands_inner!(UserEntity, "users");
```

This generates: `entity_create`, `entity_read`, `entity_update`, `entity_delete`, `entity_list`, `entity_count`.

## License

Proprietary — internal use only.
