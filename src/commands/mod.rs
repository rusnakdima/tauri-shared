pub mod algorithm_commands;
pub mod crud_commands;
pub mod crud_macro;
pub mod logger_commands;
pub mod schema_commands;
pub mod schema_sync_commands;
pub mod update_commands;

pub use algorithm_commands::{algo_execute, execute_algorithm, list_algorithms};
pub use crud_commands::crud_execute;
pub use schema_commands::{
  delete_schema, get_all_schemas, get_schema_direct, get_ui_schema, save_schema, save_ui_schema,
};
pub use schema_sync_commands::get_schema;
pub use update_commands::{
  check_for_update_command, download_update_command, get_current_version, install_update_command,
};
