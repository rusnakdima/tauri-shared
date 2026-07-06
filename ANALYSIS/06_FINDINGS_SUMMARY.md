# SDUI Migration Planning - Key Findings

## tauri-shared Library

### What It Provides

#### Data Structures
- **UiSchema** - Root SDUI schema (app, pages, layouts, components, services, modules, i18n)
- **Page/Layout/Component** - UI composition structures
- **CanvasElement** - Renderable elements with grid positioning
- **DataBinding** - Entity-field binding for reactive data
- **SignalStore** - In-memory reactive state with subscriptions
- **RBAC entities** - Role, Permission, AppUser, Session

#### Business Logic Algorithms
- **Sorting algorithms** - quick_sort, merge_sort, bubble_sort, insertion_sort
- **Graph algorithms** - dijkstra shortest path
- **Template resolution** - `{{data.field}}` and `{{env.VAR}}` binding
- **i18n translation** - Key-based locale translation
- **Schema validation** - UiSchema structure validation

#### IPC Commands (Tauri)
- Schema loading/rendering
- Data binding resolution
- Permission checking
- Cloud sync trigger

#### Storage
- **JsonDb** - Simple file-based JSON storage
- **JsonProvider** - Wrapper around nosql_orm::JsonProvider
- **SignalStore** - In-memory reactive store

#### Sync
- **SyncEngine** - Local-first sync with change detection
- **SchemaSyncService** - Schema versioning and storage

### RBAC Implementation
Located at `src/rbac/`:
- Password hashing with simple hash (NOT cryptographically secure)
- Session token generation (not JWT)
- login/logout/register functions
- Permission matching by resource + action

## nosql_orm Library

### What It Provides

#### ORM Features
- **Entity derive macro** - Declarative entity definition
- **Repository pattern** - Generic CRUD operations
- **Query builder** - Type-safe filtering, ordering, pagination
- **Relations** - One-to-many, many-to-one, one-to-one, many-to-many
- **Migrations** - Versioned migrations with up/down
- **Soft delete** - Automatic deleted_at filtering
- **Timestamps** - created_at/updated_at automatic management
- **Validators** - Email, Length, Pattern, Range

#### Storage Providers
| Provider | Feature Flag | Storage |
|----------|--------------|---------|
| JsonProvider | `json` | JSON files |
| MongoProvider | `mongo` | MongoDB |
| RedisProvider | `redis` | Redis |
| PostgresProvider | `sql-postgres` | PostgreSQL |
| SqliteProvider | `sql-sqlite` | SQLite |
| MySqlProvider | `sql-mysql` | MySQL |

#### JSON Provider Details
- Collection = JSON file: `<base_dir>/<collection>.json`
- In-memory cache with async RwLock
- LRU eviction (has known memory leak bug)
- Document ID auto-generation
- Patch with $inc support
- Transaction support (single-doc locking)

## Integration Between Libraries

### Current State
tauri-shared DEPENDS on nosql_orm:
```toml
nosql_orm = { path = "../nosql_orm", features = ["json"] }
```

### Integration Points
1. **Entity impl** - tauri-shared types implement nosql_orm::Entity
2. **Error mapping** - OrmError -> AppError
3. **JsonProvider wrapper** - For Tauri state management
4. **DatabaseProvider usage** - For auth operations

## Migration Opportunities

### Replace tauri-shared JsonDb with nosql_orm

**Current JsonDb** (simple, limited):
- No query builder
- No relation support
- No migration system
- Single-file storage

**Should use nosql_orm Repository**:
```rust
// Instead of:
let db = JsonDb::new("./data")?;

// Use:
let provider = JsonProvider::new("./data").await?;
let repo = Repository::<UiSchema, _>::new(provider);
```

### Schema Storage Migration
| Current | Target |
|---------|--------|
| JsonDb with app_id collection | Repository<UiSchema, JsonProvider> |
| Manual find/update | repo.save(), repo.find_by_id() |
| No versioning | MigrationRunner for schema versions |

### Auth Storage Migration
| Current | Target |
|---------|--------|
| Manual JSON find_all | Repository<AppUser, JsonProvider> |
| Simple password hash | Keep, or upgrade to bcrypt |
| Session in JSON | Repository<Session, JsonProvider> |

## Business Logic Duplication

### Algorithm Duplication
- tauri-shared has BOTH:
  - `src/algorithms/sorting.rs` - Generic implementations
  - `src/commands/algorithm_commands.rs` - JSON-aware versions
  
- The JSON versions duplicate logic with custom `json_ord()` for Value comparison
- SOLUTION: Make generic algorithms work with `dyn Ord` or provide JSON wrapper

### Sync Logic
- tauri-shared has its own sync engine in `src/sync/local_first.rs`
- Uses simple JsonDb, not nosql_orm
- Could leverage nosql_orm's CDC feature for change detection

## SDUI Schema Storage for JSON Provider

### Recommended Schema Definition
```rust
// SDUI Schema as nosql_orm Entity
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoredSchema {
    pub id: Option<String>,
    pub app_id: String,
    pub version: String,
    pub schema: UiSchema,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Entity for StoredSchema {
    fn meta() -> EntityMeta { EntityMeta::new("stored_schemas") }
    // ...
}
```

### Benefits
1. **Version tracking** - Each schema version stored
2. **Query** - Find schemas by app_id, version range
3. **Migrations** - Add fields via MigrationRunner
4. **Sync** - Leverage existing change detection

## Known Issues (from CODEBASE_ANALYSIS.md)

### nosql_orm Issues
1. **JsonProvider LRU leak** - access_order not pruned on eviction
2. **PooledJson::drop() empty** - Resource leak
3. **Redis N+1 queries** - Inefficient batch operations
4. **No index implementation** - JsonProvider create_index is no-op

### tauri-shared Issues
1. **Password hash weak** - Using simple hasher, not bcrypt
2. **Session tokens not JWT** - No expiration verification
3. **Sync uses custom engine** - Could leverage nosql_orm CDC

## Action Items for Migration

1. [ ] Migrate schema storage from JsonDb to Repository<UiSchema, JsonProvider>
2. [ ] Migrate auth storage to nosql_orm with proper relations
3. [ ] Implement proper password hashing (bcrypt/argon2)
4. [ ] Use MigrationRunner for schema versioning
5. [ ] Consider using nosql_orm CDC instead of custom sync
6. [ ] Fix JsonProvider LRU eviction bug
7. [ ] Consolidate duplicate sorting algorithm implementations
