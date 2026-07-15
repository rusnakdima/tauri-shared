# tauri-shared

Rust backend library for Tauri desktop applications. Provides schema management, storage (JsonProvider + SignalStore), SchemaSyncService, RBAC, logging, HTTP/WebSocket clients, auto-update, and algorithm implementations.

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
use tauri_shared::commands::{
    db_get_schema, db_save_schema, db_get_all_schemas, db_delete_schema,
    get_schema, save_schema, get_ui_schema, save_ui_schema, delete_schema,
};
```

| Command | Signature | Description |
|---|---|---|
| `db_get_schema` | `(id: String) -> Result<UiSchema>` | Fetch schema by ID from nosql_orm |
| `db_save_schema` | `(id: String, schema: UiSchema) -> Result<()>` | Save schema to nosql_orm |
| `db_get_all_schemas` | `() -> Result<Vec<UiSchema>>` | List all schemas |
| `db_delete_schema` | `(id: String) -> Result<()>` | Delete schema by ID |
| `get_schema` | `(id: String) -> Result<UiSchema>` | Fetch schema (alternative API) |
| `save_schema` | `(schema: UiSchema) -> Result<()>` | Save schema (alternative API) |

### Schema Sync Commands

```rust
use tauri_shared::commands::schema_sync_commands::{get_schema_local_first, sync_schema_from_cloud};
```

| Command | Signature | Description |
|---|---|---|
| `get_schema_local_first` | `(id: String) -> Result<Response<Value>>` | Try local JSON first, fall back to MongoDB cloud |
| `sync_schema_from_cloud` | `(id: String) -> Result<Response<Value>>` | Force sync from MongoDB to local JSON |

### RBAC Commands

```rust
use tauri_shared::commands::check_permission;
```

| Command | Signature | Description |
|---|---|---|
| `check_permission` | `(permission: Permission, resource: String, action: String) -> Result<bool>` | Check if permission matches resource/action |

### Logger Commands

```rust
use tauri_shared::commands::logger_commands::{get_log_entries, get_log_level, set_log_level, clear_logs, write_log_to_file};
```

### Algorithm Commands

Requires `features = ["algorithms"]`:

```rust
use tauri_shared::algorithms::{ValidationAlgorithm, SearchAlgorithm};

// Validation
ValidationAlgorithm::validate_input("foo", 10);
ValidationAlgorithm::validate_email("test@example.com");
ValidationAlgorithm::sanitize_input("hello world");

// Search
SearchAlgorithm::search_schemas(&["schema1", "schema2"], "schema1");
SearchAlgorithm::paginate(&[1, 2, 3, 4, 5], 1, 2); // page 1, limit 2 → [1, 2]
```

## Schema System

The schema system manages UI schemas for the application:

### Core Types

```rust
use tauri_shared::schema::{UiSchema, Page, Layout, LayoutElement, Component, DataBinding};

pub struct UiSchema {
    pub pages: Vec<Page>,
    pub navigation: Vec<NavItem>,
    pub theme: Option<ThemeConfig>,
    pub globals: Option<serde_json::Value>,
}

pub struct Page {
    pub id: String,
    pub title: String,
    pub route: String,
    pub layout: Option<Layout>,
    pub elements: Vec<LayoutElement>,
    pub condition: Option<String>,
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

### Grid System

```rust
use tauri_shared::schema::grid::{GridPosition, ResponsiveBreakpoints, ResponsiveClasses, NamedGridArea, GridElement};

let pos = GridPosition {
    column: Some("1".to_string()),
    row: Some("1".to_string()),
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

### SchemaSyncService

Sync schemas between local JSON files and MongoDB cloud:

```rust
use tauri_shared::storage::{SchemaSyncService, SchemaSyncState, SchemaConfig};

let sync_service = SchemaSyncService::new(config, provider);
sync_service.sync_schema("my-schema").await?;
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

### Response Constructors

```rust
Response::success(data, Some("message"));
Response::created(data);
Response::updated(data);
Response::deleted(data);
Response::error("Something went wrong");
Response::error_with_data(data, "Error with data");
Response::validation_error("Invalid input");
Response::not_found("Item not found");
Response::unauthorized();
Response::forbidden();
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

## HTTP Client

```rust
use tauri_shared::http_client::{HttpClient, HttpResponse};

let client = HttpClient::new("https://api.example.com".to_string());
let response: HttpResponse<serde_json::Value> = client.get("/endpoint").await?;
```

## WebSocket Client

```rust
use tauri_shared::websocket::WsClient;

let ws = WsClient::new("wss://api.example.com/ws".to_string());
ws.send("hello".to_string()).await?;
let msg = ws.receive().await?;
```

## Auto-Update

```rust
use tauri_shared::update::{
    check_for_update, download_update, install_update,
    check_for_update_command, get_current_version,
};

let update_info = check_for_update().await?;
let version = get_current_version();
```

## Extension System (Designer)

```rust
use tauri_shared::extension::{DesignerExtension, designer_extension, init_extensions_with_app};

pub struct MyExtension;

impl DesignerExtension for MyExtension {
    fn name(&self) -> &str { "my-extension" }
    fn register_commands(&self, app: &mut tauri::App) {
        app.register_command("myCommand", |app, args| async move {
            Ok(serde_json::json!({ "result": "success" }))
        });
    }
}

designer_extension!(MyExtension);

// In your app setup:
init_extensions_with_app(&mut app);
```

## Algorithms (feature = "algorithms")

Requires `features = ["algorithms"]` in Cargo.toml.

### Validation

```rust
use tauri_shared::algorithms::ValidationAlgorithm;

ValidationAlgorithm::validate_input("hello", 10);  // true
ValidationAlgorithm::validate_email("test@example.com");  // true
ValidationAlgorithm::sanitize_input("hello world!");  // "hello world"
```

### Search

```rust
use tauri_shared::algorithms::SearchAlgorithm;

let items = vec!["schema1", "schema2", "schema3"];
let results = SearchAlgorithm::search_schemas(&items, "schema1");  // ["schema1"]

let paginated = SearchAlgorithm::paginate(&[1, 2, 3, 4, 5], 1, 2);  // [1, 2]
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

## License

Proprietary — internal use only.
