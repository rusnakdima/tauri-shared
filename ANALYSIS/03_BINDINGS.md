# TypeScript Bindings

## Location
`/mnt/Other/Projects/Rust/tauri-shared/bindings/`

## Generation
Uses `ts-rs` crate with `#[ts(export)]` derive macro on Rust structs.
Generator: `/mnt/Other/Projects/Rust/tauri-shared/src/typescript/generator.rs`

## Binding Files

### Schema Core
- `UiSchema.ts` - Root schema interface
- `Page.ts` - Page definition
- `PageSection.ts` - Section within page
- `PageMeta.ts` - Page metadata (title, icon, breadcrumb)
- `Layout.ts` - Layout structure
- `LayoutSlot.ts` - Slot in layout

### Components
- `ComponentDef.ts` - Component definition
- `ComponentProp.ts` - Component property
- `ComponentEvent.ts` - Component event

### Services
- `ServiceDef.ts` - Service definition
- `ServiceCrud.ts` - CRUD configuration
- `ServiceField.ts` - Service field

### Grid System
- `GridPosition.ts` - Element position
- `GridDefaults.ts` - Default grid values
- `GridTemplate.ts` - Grid template
- `GridTrack.ts` - Grid track (column/row)
- `GridArea.ts` - Named grid area
- `CanvasElement.ts` - Canvas element

### Data Binding
- `DataBinding.ts` - Entity + field binding

### Theme/i18n
- `Theme.ts` - Theme colors
- `ThemeColors.ts` - Color definitions
- `ColorMode.ts` - Light/Dark/System
- `I18nConfig.ts` - i18n configuration
- `LocaleMap.ts` - Locale translations

### App Config
- `AppConfig.ts` - App configuration
- `AppSettings.ts` - App settings

### Modules/Commands
- `ModuleDef.ts` - Module definition
- `CommandDef.ts` - Command definition
- `MiddlewareDef.ts` - Middleware definition

### Tailwind
- `TailwindBreakpoints.ts` - Breakpoint definitions
- `TailwindResponsiveClasses.ts` - Responsive classes
- `TailwindGridElement.ts` - Grid element
- `TailwindGridArea.ts` - Grid area
