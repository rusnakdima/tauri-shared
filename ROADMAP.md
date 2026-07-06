# ROADMAP.md — tauri-shared (Rust)

## Vision

Provides ALL backend logic, algorithms, data types, and Tauri command helpers for schema-driven apps. Apps import shared logic, not duplicate it.

---

## Library Structure

```
tauri-shared/src/
├── lib.rs                    # Main entry (re-exports everything)
├── response.rs               # Status + Response<T> builder
├── entity.rs                 # Entity/EntityMeta for DB models
├── sdui/
│   ├── mod.rs
│   ├── schema.rs             # UiSchema, Page, CanvasElement, Layout
│   ├── engine.rs             # SduiEngine — loads/renders schemas
│   ├── binding.rs            # DataBinding resolution
│   └── events.rs             # Event dispatch
├── rbac/
│   ├── mod.rs
│   ├── role.rs               # Role, RolePermission
│   ├── permission.rs         # Permission, PermissionMatch
│   ├── auth.rs               # User, Session, hash_password, verify_password
│   └── service.rs            # RbacService — thin wrapper
├── storage/
│   ├── mod.rs
│   ├── signal_store.rs       # SignalStore (reactive state)
│   ├── json_db.rs            # JsonDb (file-based JSON storage)
│   └── cache.rs              # StorageCacheService
├── sync/
│   ├── mod.rs
│   ├── sync_engine.rs        # SyncEngine
│   ├── sync_queue.rs         # SyncQueue
│   └── mongo_bridge.rs       # MongoBridge HTTP client
├── validation/
│   ├── mod.rs
│   ├── name.rs               # validate_name()
│   ├── sql.rs                # validate_sql()
│   └── conn_id.rs            # validate_conn_id()
├── algorithms/
│   ├── mod.rs
│   ├── sorting.rs            # quick_sort, merge_sort, bubble_sort, insertion_sort
│   └── graph.rs              # dijkstra_shortest_path (feature-gated)
└── crud/
    ├── mod.rs
    └── query.rs              # PaginatedResult, CrudQuery, CrudFilter
```

---

## Implemented Features

### Response Pattern
- [x] Status enum: Success, Error, NotFound, Unauthorized, ValidationError
- [x] Response<T> with builder: `Response::success(data, Some(msg))`, `Response::error(msg)`
- [x] Result<T> type alias

### Entity/ORM
- [x] Entity trait with EntityMeta::new("table_name") — NOT #[derive(Entity)]
- [x] ts-rs for TypeScript bindings
- [x] JsonProvider (async) for JSON file storage

### Schema-Driven UI
- [x] Full schema types: UiSchema, Page, Route, Layout, CanvasElement, DataBinding
- [x] SduiEngine: load_schema, render_page, resolve_binding, dispatch_event
- [x] Component manifest types (SharedComponentDef, SharedComponentProp)

### RBAC
- [x] Role, RolePermission, Permission types
- [x] User/Session with password hashing
- [x] Permission::matches for access checks
- [x] RbacService thin-wrapper

### Storage
- [x] SignalStore with reactive state
- [x] JsonDb with collections
- [x] StorageCacheService (in-memory LRU)
- [x] MongoBridge for cloud sync

### Sync
- [x] SyncEngine with queue
- [x] SyncOperation (Insert/Update/Delete)
- [x] MongoDB HTTP bridge

### Validation
- [x] validate_name (length ≤255, no path chars, no SQL injection)
- [x] validate_sql (whitelist: SELECT, INSERT, UPDATE, DELETE, etc.)
- [x] validate_conn_id (UUID format)

### Algorithms (feature-gated)
- [x] Sorting: quick_sort, merge_sort, bubble_sort, insertion_sort
- [x] Graph: dijkstra_shortest_path

### CRUD
- [x] PaginatedResult<T>
- [x] CrudQuery, CrudFilter
- [x] CrudResult<T>

---

---

## Migration to Unified Architecture

### Phase 1: Database Integration via nosql_orm ✅ COMPLETED 2026-07-05

All database operations should go through `nosql_orm`:

- [x] `KernelDb` trait in `commands/mod.rs` defines database interface
- [x] `KernelEntity` trait for serializable entities
- [x] All commands use `impl DatabaseProvider` in Tauri State

### Phase 2: SDUI Runtime ✅ COMPLETED 2026-07-05

- [x] `SduiEngine` - Central runtime engine
- [x] `SchemaRouter` - Route resolution with guards/middleware
- [x] `BindingResolver` - Data binding resolution
- [x] `sdui_commands.rs` - Tauri command handlers

### Phase 3: Algorithm Consolidation 🔲 PENDING

- [x] Algorithms exist in `algorithms/` module
- [x] Feature-gated graph algorithms (requires petgraph)
- [ ] Frontend should call via invoke, not duplicate implementations

---

## Pending Tasks

### Step 1: Missing Command Exposure ✅ COMPLETED 2026-06-28

SduiEngine logic exists but no tauri commands exposed for runtime invocation.

- [x] Add `#[tauri::command]` for:
  - [x] `load_schema` → SduiEngine::load_schema
  - [x] `render_page` → SduiEngine::render_page
  - [x] `resolve_binding` → SduiEngine::resolve_binding
  - [x] `sync_to_cloud` → SyncEngine::sync_to_cloud
  - [x] `check_permission` → Permission::matches wrapper

### Step 2: algorithms-manifest.json ✅ COMPLETED 2026-06-28

- [x] Generate algorithms-manifest.json (sorting, graph algorithms)
- [x] File: `/mnt/Other/Projects/Rust/tauri-shared/algorithms-manifest.json`
- [x] Include: quick_sort, merge_sort, bubble_sort, insertion_sort, dijkstra

### Step 3: i18n Support ✅ COMPLETED 2026-06-28

- [x] I18nConfig already defined in schema/i18n.rs
- [x] Add `fn translate(key: &str, locale: &str) -> String`
- [x] Add `#[tauri::command]` for runtime translation

**Created:** `src/i18n/mod.rs`, `src/i18n/translate.rs`

### Step 4: Logger Command Integration ✅ COMPLETED 2026-06-28

- [x] Add `write_log_to_file` Tauri command to expose FileLogger to TypeScript

**Created:** `src/commands/logger_commands.rs`

### Step 5: Feature Gate Cleanup ✅ COMPLETED 2026-06-28

algorithms feature-gated — sorting now always available:
- [x] Always compile sorting algorithms (quick_sort, merge_sort, bubble_sort, insertion_sort)
- [x] Keep graph algorithms feature-gated (dijkstra requires petgraph)

**Fixed:** Pre-existing type bug in `src/algorithms/graph.rs` - `dijkstra_shortest_path` return type corrected from `Vec<Option<f64>>` to `HashMap<NodeIndex, f64>`

### Step 6: Thin-Wrapper Pattern for Apps ✅ VERIFIED 2026-06-28

All Tauri commands in consumer apps should be thin wrappers delegating to shared logic.

- [x] ZenithDB already uses thin-wrapper pattern via `models/response.model.rs` re-exporting `tauri_shared::Response`
- [x] ZenithDB commands in `src-tauri/src/commands/` delegate to services
- [x] Pattern documented in AGENTS.md conventions

---

## Key Rules

### File Naming
- `name_type.rs` — NOT `name.type.rs`
- mod.rs: ONLY re-exports, NO code

### Response Usage
```rust
// CORRECT
Response::success(user, Some("User retrieved"))
Response::error("Invalid input你了")

// WRONG
Response { status: Status::Success, data: Some(user), message: Some("msg".to_string()) }
```

### Entity Definition
```rust
// CORRECT
impl Entity for User {
    fn meta() -> EntityMeta { EntityMeta::new("users") }
}

// WRONG (conflicts with ts-rs)
#[derive(Entity)]
struct User { ... }
```

### Field Naming
- ALL fields: snake_case
- `pub first_name: String` NOT `pub firstName: String`

---

## Build Commands

```bash
cd /mnt/Other/Projects/Rust/tauri-shared && cargo build

# With features
cargo build --features algorithms
```
