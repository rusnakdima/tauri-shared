# SDUI Runtime Analysis

## SduiEngine (src/runtime/engine.rs)

The SduiEngine is the core runtime for executing SDUI schemas.

### Structure
```rust
pub struct SduiEngine {
    pub schema: UiSchema,
    pub theme: Theme,
    pub store: Arc<SignalStore>,      // Reactive state
    pub functions: HashMap<String, Callable>,  // Registered JS functions
}
```

### Key Methods
| Method | Purpose |
|--------|---------|
| `load_schema()` | Load/replace UiSchema |
| `get_page()` | Find page by ID |
| `get_layout()` | Find layout by ID |
| `get_component()` | Find component by ID |
| `register_function()` | Register callable function |
| `call_function()` | Invoke registered function |
| `get/set/update/delete_state()` | State management |
| `subscribe_to_state()` | React to state changes |
| `resolve_binding()` | Resolve DataBinding to value |
| `render_page()` | Render page with resolved bindings |
| `validate()` | Validate schema |

### Render Flow
```rust
pub fn render_page(&self, route: &str) -> Result<RenderedPage> {
    let page = self.get_page(route)?;
    let mut rendered = RenderedPage::from_page(page, None);
    
    for element in &mut rendered.canvas_elements {
        if let Some(ref binding) = element.data_binding {
            if let Ok(value) = self.resolve_binding(binding) {
                element.resolved_props = Some(value);
            }
        }
    }
    Ok(rendered)
}
```

## SignalStore (src/storage/signal_store.rs)

Reactive in-memory store for UI state.

### Interface
```rust
pub struct SignalStore {
    data: RwLock<HashMap<String, Value>>,
    subscribers: RwLock<Vec<Box<dyn Fn(&str, &Value) + Send + Sync>>>,
}

impl SignalStore {
    pub fn set(&self, key: &str, value: Value);      // Set + notify
    pub fn get(&self, key: &str) -> Option<Value>;
    pub fn update(&self, key: &str, f: FnOnce(&Value) -> Value);  // Atomic update
    pub fn delete(&self, key: &str);
    pub fn keys(&self) -> Vec<String>;
    pub fn subscribe<F>(&self, callback: F);  // Subscribe to changes
    pub fn to_json(&self) -> Value;
    pub fn from_json(&self, json: Value);
}
```

### Subscription Pattern
When state changes, all subscribers are notified:
```rust
fn notify(&self, key: &str, value: &Value) {
    for subscriber in subscribers.iter() {
        subscriber(key, value);
    }
}
```

## BindingResolver (src/runtime/bindings.rs)

Resolves template expressions in strings.

### Template Syntax
```
{{data.field}}      - Data binding
{{env.VAR}}         - Environment variable
```

### Example
```rust
let template = "Hello, {{data.name}}!";
let context = serde_json::json!({"name": "Alice"});
resolver.resolve(&template, &context)?;  // "Hello, Alice!"
```

## Schema Structure (src/schema/mod.rs)

### UiSchema Root
```rust
pub struct UiSchema {
    pub schema_version: String,
    pub app: AppConfig,
    pub pages: Vec<Page>,
    pub layouts: Vec<Layout>,
    pub components: Vec<ComponentDef>,
    pub shared_components: Vec<ComponentDef>,
    pub services: Vec<ServiceDef>,
    pub modules: Vec<ModuleDef>,
    pub i18n: I18nConfig,
}
```

### Page Structure
```rust
pub struct Page {
    pub id: String,
    pub name: String,
    pub route: String,
    pub layout: Option<String>,           // Layout ID
    pub meta: PageMeta,
    pub sections: HashMap<String, PageSection>,
    pub canvas_elements: Vec<CanvasElement>,
}
```

### CanvasElement
```rust
pub struct CanvasElement {
    pub id: String,
    pub component_id: String,
    pub grid_position: GridPosition,
    pub props: HashMap<String, serde_json::Value>,
    pub classes: String,
    pub children: Vec<String>,            // Child element IDs
    pub data_binding: Option<DataBinding>,
}
```

## SDUI Command Handlers (src/commands/sdui_commands.rs)

| Command | Handler | Purpose |
|---------|---------|---------|
| `load_schema` | `load_schema()` | Load schema into engine |
| `render_page` | `render_page()` | Render page by route |
| `resolve_binding` | `resolve_binding()` | Resolve data binding |
| `sync_to_cloud` | `sync_to_cloud()` | Sync to MongoDB |
| `check_permission` | `check_permission()` | RBAC check |

## Sync Architecture (src/sync/local_first.rs)

### SyncEngine
```rust
pub struct SyncEngine {
    queue: SyncQueue,
    local_db: JsonDb,
    mongo_bridge: Option<MongoBridge>,
    last_sync_state: RwLock<HashMap<String, HashMap<String, Value>>>,
}
```

### Sync Flow
1. Track local changes with timestamps
2. Detect creates/updates/deletes vs last sync
3. Queue operations
4. Replay to MongoDB via bridge

## Theme System (src/schema/theme.rs)

Theme colors for light/dark modes:
- `get_light_theme()`
- `get_dark_theme()`
- Applied via SduiEngine
