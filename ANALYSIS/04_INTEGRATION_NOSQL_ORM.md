# tauri-shared x nosql_orm Integration

## Dependencies
tauri-shared Cargo.toml:
```toml
nosql_orm = { path = "../nosql_orm", features = ["json"] }
```

## Integration Points

### 1. Entity Trait Usage
tauri-shared implements `Entity` for its types:

**UiSchema entity** (`src/schema/entity.rs`):
```rust
impl Entity for UiSchema {
    fn meta() -> EntityMeta { EntityMeta::new("ui_schemas") }
    fn get_id(&self) -> Option<String> { Some(self.app.id.clone()) }
    fn set_id(&mut self, id: String) { self.app.id = id; }
}
```

**RBAC entities** (`src/rbac/roles.rs`):
- `Role`, `UserRole`, `Permission`, `RolePermission`, `AppUser`, `Session`

### 2. DatabaseProvider Usage
tauri-shared uses `DatabaseProvider` in:
- `KernelDb` trait (`src/commands/mod.rs`)
- `login()`, `logout()`, `register()` in `src/rbac/auth.rs`
- Schema storage commands

```rust
pub trait KernelDb: Send + Sync {
    fn find_by_id(&self, collection: &str, id: &str) -> impl Future<Output = OrmResult<Option<Value>>>;
    fn find_all(&self, collection: &str) -> impl Future<Output = OrmResult<Vec<Value>>>;
    fn insert(&self, collection: &str, data: Value) -> impl Future<Output = OrmResult<()>>;
    fn update(&self, collection: &str, id: &str, data: Value) -> impl Future<Output = OrmResult<()>>;
    fn delete(&self, collection: &str, id: &str) -> impl Future<Output = OrmResult<()>>;
}
```

### 3. Error Translation
`src/error.rs` maps nosql_orm errors to AppError:
```rust
impl From<nosql_orm::error::OrmError> for AppError {
    fn from(err: nosql_orm::error::OrmError) -> Self {
        match err {
            OrmError::NotFound(entity) => AppError::NotFound(entity),
            OrmError::Validation(msg) => AppError::ValidationError(msg),
            OrmError::Duplicate(entity) => AppError::Duplicate(entity),
            // ... etc
        }
    }
}
```

### 4. JSON Provider Wrapper
`src/storage/json_provider.rs` wraps nosql_orm's JsonProvider for Tauri state:
```rust
pub type JsonProviderState = Arc<JsonProvider>;

pub async fn create_json_provider(data_dir: impl AsRef<Path>) -> OrmResult<JsonProvider> {
    JsonProvider::new(data_dir).await
}
```

### 5. Schema Sync
`src/sync/schema_sync.rs` uses JsonDb (NOT nosql_orm) for local schema storage:
```rust
pub struct SchemaSyncService {
    local_db: JsonDb,  // tauri-shared's simple JSON DB
}
```
Alternative: Could use nosql_orm for better query support.

## What Could Use nosql_orm

### Current tauri-shared JsonDb (simple)
Located at `src/storage/json_db.rs`:
- Simple HashMap<String, HashMap<String, Value>>
- Single file per "database"
- No query builder
- No relations

### Should Migrate to nosql_orm for:
1. **Schema storage** - Use Repository<UiSchema, JsonProvider>
2. **User/session storage** - Repository<AppUser, JsonProvider>
3. **RBAC data** - Repository<Permission, JsonProvider>
4. **Query capabilities** - Filter, order, paginate
5. **Migration support** - Schema versioning

## Proposed Migration

### Current (tauri-shared JsonDb):
```rust
let db = JsonDb::new("./data")?;
db.insert("schemas", "app1", schema)?;
let schema = db.find("schemas", "app1")?;
```

### Proposed (nosql_orm):
```rust
let provider = JsonProvider::new("./data").await?;
let repo: Repository<UiSchema, _> = Repository::new(provider);
repo.save(schema).await?;
let schema = repo.find_by_id("app1").await?;
```
