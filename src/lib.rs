pub mod error;
pub mod macros;
pub mod response;
pub mod result;

pub use error::AppError;
pub use macros::impl_entity_commands_inner;
pub use response::{Response, Status};
pub use result::{OrmResult, Result};

#[macro_export]
macro_rules! impl_entity_commands {
    ($entity:ident) => {
        $crate::macros::commands::impl_entity_commands_inner(stringify!($entity))
    };
}
