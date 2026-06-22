pub mod crud;
pub mod error;
pub mod logger;
pub mod macros;
pub mod response;
pub mod result;
pub mod schema;
pub mod storage;

pub use schema::*;

pub use crud::{CrudFilter, CrudQuery, CrudResult};
pub use error::AppError;
pub use logger::{Logger, LogEntry, LogLevel};
pub use macros::impl_entity_commands_inner;
pub use response::{Response, Status};
pub use result::{OrmResult, Result};
pub use storage::{JsonDb, SignalStore};

#[macro_export]
macro_rules! impl_entity_commands {
    ($entity:ident) => {
        $crate::macros::commands::impl_entity_commands_inner(stringify!($entity))
    };
}
