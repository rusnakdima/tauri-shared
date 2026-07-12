//! Extension system for Designer IPC commands
//!
//! This module allows `@Designer/` libraries to extend the IPC command API
//! without modifying the main Designer application.
//!
//! # How it works
//!
//! 1. A library implements `DesignerExtension` trait with custom IPC commands
//! 2. The library calls `designer_extension!(MyExtension)` to register itself
//! 3. The main Designer app calls `init_extensions_with_app(&mut app)` during setup
//! 4. All registered extensions have their commands dynamically registered via `app.register_command()`
//!
//! # Example
//!
//! ```rust,ignore
//! // In your library's lib.rs
//! use tauri_shared::extension::{DesignerExtension, designer_extension};
//!
//! pub struct MyExtension;
//!
//! impl DesignerExtension for MyExtension {
//!     fn name(&self) -> &str { "my-extension" }
//!
//!     fn register_commands(&self, app: &mut tauri::App) {
//!         app.register_command("myCommand", |app, args: serde_json::Value| async move {
//!             // Handle the command
//!             Ok(serde_json::json!({ "result": "success" }))
//!         });
//!     }
//! }
//!
//! designer_extension!(MyExtension);
//! ```

use std::sync::Arc;
use tauri::AppHandle;

/// Trait for Designer extensions that provide custom IPC commands.
///
/// Libraries implementing this trait can add their own commands to the
/// Designer's IPC API without modifying the core application.
pub trait DesignerExtension: Send + Sync {
    /// Unique name identifying this extension.
    fn name(&self) -> &str;

    /// Register custom IPC commands with the Tauri app.
    ///
    /// Use `app.register_command()` to add commands that will be
    /// accessible from the frontend via `window.ipc.invoke()`.
    fn register_commands(&self, _app: &mut tauri::App) {}

    /// Initialize the extension (called once when the app starts).
    /// Use this to set up any state or resources needed by the extension.
    fn init(&self, _app: &AppHandle) {}
}

/// Global registry of Designer extensions.
/// Uses a static with Mutex to allow libraries to register themselves at library load time.
static EXTENSION_REGISTRY: std::sync::LazyLock<std::sync::Mutex<Vec<Arc<dyn DesignerExtension>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

/// Register an extension with the global registry.
///
/// This is typically called automatically by the `designer_extension!` macro,
/// but can be called manually if needed.
pub fn register_extension<E: DesignerExtension + 'static>(extension: E) {
    let ext: Arc<dyn DesignerExtension> = Arc::new(extension);
    let mut registry = EXTENSION_REGISTRY.lock().unwrap();
    // Avoid duplicates
    if !registry.iter().any(|e| e.name() == ext.name()) {
        registry.push(ext);
    }
}

/// Initialize all registered extensions with the Tauri app.
///
/// Call this function during the Designer's setup phase to register
/// all extension commands with the app.
///
/// # Example
///
/// ```rust,ignore
/// use tauri_shared::extension::init_extensions_with_app;
///
/// tauri::Builder::default()
///     .setup(|app| {
///         init_extensions_with_app(app);
///         Ok(())
///     })
/// ```
pub fn init_extensions_with_app(app: &mut tauri::App) {
    // Get extensions from registry
    let extensions: Vec<Arc<dyn DesignerExtension>> = {
        let registry = EXTENSION_REGISTRY.lock().unwrap();
        registry.clone()
    };

    // Initialize each extension
    for ext in extensions.iter() {
        ext.register_commands(app);
        ext.init(&app.handle().clone());
    }
}

/// Get all registered extension names.
pub fn get_extension_names() -> Vec<String> {
    let registry = EXTENSION_REGISTRY.lock().unwrap();
    registry.iter().map(|e| e.name().to_string()).collect()
}

/// Macro to automatically register a Designer extension.
///
/// This macro should be called once in your library's code to register
/// the extension with the global registry. It creates a static instance
/// of your extension and registers it.
///
/// # Example
///
/// ```rust,ignore
/// use tauri_shared::extension::{DesignerExtension, designer_extension};
///
/// pub struct MyExtension;
///
/// impl DesignerExtension for MyExtension {
///     fn name(&self) -> &str { "my-extension" }
///     fn register_commands(&self, app: &mut tauri::App) {
///         app.register_command("myCommand", |app, args| async move {
///             Ok(serde_json::json!({ "data": "test" }))
///         });
///     }
/// }
///
/// designer_extension!(MyExtension);
/// ```
#[macro_export]
macro_rules! designer_extension {
    ($extension:expr) => {
        // Create a static to ensure the extension lives for the lifetime of the program
        static EXT: std::sync::OnceLock<std::sync::Arc<dyn $crate::extension::DesignerExtension>> =
            std::sync::OnceLock::new();
        EXT.get_or_init(|| {
            std::sync::Arc::new($extension)
        });
        // Register with the global registry
        $crate::extension::register_extension($extension);
    };
}
