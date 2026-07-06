use std::future::Future;

pub trait KernelDb: Send + Sync {
    fn find_by_id(
        &self,
        collection: &str,
        id: &str,
    ) -> impl Future<
        Output = std::result::Result<Option<serde_json::Value>, nosql_orm::error::OrmError>,
    > + Send;

    fn find_all(
        &self,
        collection: &str,
    ) -> impl Future<Output = std::result::Result<Vec<serde_json::Value>, nosql_orm::error::OrmError>>
           + Send;

    fn insert(
        &self,
        collection: &str,
        data: serde_json::Value,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;

    fn update(
        &self,
        collection: &str,
        id: &str,
        data: serde_json::Value,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;

    fn delete(
        &self,
        collection: &str,
        id: &str,
    ) -> impl Future<Output = std::result::Result<(), nosql_orm::error::OrmError>> + Send;
}