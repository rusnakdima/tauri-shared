# tauri-shared Structure

## Source Layout
```
src/
├── lib.rs                    # Main entry, re-exports all modules
├── algorithms/               # Sorting & graph algorithms
│   ├── mod.rs
│   ├── sorting.rs           # quick_sort, merge_sort, bubble_sort, insertion_sort
│   └── graph.rs             # dijkstra (petgraph feature-gated)
├── commands/                # Tauri IPC handlers
│   ├── mod.rs               # KernelEntity, KernelDb traits
│   ├── algorithm_commands.rs # JSON-aware algorithm commands
│   ├── sdui_commands.rs      # load_schema, render_page, resolve_binding
│   └── schema_commands.rs
├── crud.rs                  # PaginatedResult, CrudFilter, CrudQuery
├── error.rs                 # AppError from nosql_orm::OrmError
├── i18n/
│   ├── mod.rs
│   └── translate.rs         # translate(), tauri_translate()
├── logger/
├── lru.rs                   # Generic LRU cache
├── macros/
│   └── commands.rs          # impl_entity_commands_inner
├── migration/
├── rbac/
│   ├── mod.rs
│   ├── auth.rs             # login, logout, register, password hashing
│   ├── roles.rs            # Role, Permission, AppUser, Session
│   └── permissions.rs
├── repository/
├── response.rs              # Response<T>, Status
├── runtime/
│   ├── mod.rs
│   ├── engine.rs           # SduiEngine
│   ├── bindings.rs         # BindingResolver
│   ├── router.rs
│   └── validation.rs
├── schema/
│   ├── mod.rs              # UiSchema
│   ├── app.rs              # AppConfig, AppSettings
│   ├── page.rs             # Page, CanvasElement, GridPosition
│   ├── component.rs        # ComponentDef, ComponentProp
│   ├── layout.rs           # Layout, LayoutSlot, GridTemplate
│   ├── service.rs          # ServiceDef, ServiceCrud
│   ├── module.rs           # ModuleDef
│   ├── grid.rs             # TailwindBreakpoints
│   ├── theme.rs
│   ├── entity.rs           # Entity impl for UiSchema
│   ├── i18n.rs
│   └── runtime.rs          # RenderedPage, ValidationResult
├── storage/
│   ├── mod.rs
│   ├── json_db.rs          # JsonDb
│   ├── json_provider.rs     # nosql_orm wrapper
│   └── signal_store.rs     # Reactive store
├── sync/
│   ├── mod.rs
│   ├── schema_sync.rs      # SchemaSyncService
│   ├── local_first.rs      # SyncEngine, SyncQueue
│   └── mongo_bridge.rs
├── typescript/
│   └── generator.rs
└── validation/
    ├── sql.rs
    └── conn_id.rs
```

## Key Files Summary
| File | Purpose |
|------|---------|
| src/lib.rs | Re-exports all modules, uses nosql_orm |
| src/commands/mod.rs | KernelEntity, KernelDb traits for Tauri |
| src/schema/mod.rs | UiSchema - root SDUI structure |
| src/runtime/engine.rs | SduiEngine - schema execution runtime |
| src/storage/json_provider.rs | Wrapper for nosql_orm::JsonProvider |
| src/rbac/auth.rs | Authentication (login, register, sessions) |
| src/sync/local_first.rs | Offline-first sync engine |
