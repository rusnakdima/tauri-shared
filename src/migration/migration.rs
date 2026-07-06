use std::sync::Arc;

use nosql_orm::prelude::{DatabaseProvider, Entity};

use super::error::MigrationError;

pub struct Migration;

impl Migration {
    pub async fn auto_migrate<P, E>(
        _provider: Arc<P>,
        _entities: &[E],
    ) -> Result<(), MigrationError>
    where
        P: DatabaseProvider + 'static,
        E: Entity + Send + Sync + 'static,
    {
        Ok(())
    }
}