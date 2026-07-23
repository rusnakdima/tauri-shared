use crate::log_info;
use nosql_orm::prelude::*;
use nosql_orm::providers::json::JsonProvider;
use nosql_orm::providers::mongo::MongoProvider;

/// SchemaSyncService syncs schemas from MongoDB cloud into JsonProvider.
pub struct SchemaSyncService {
  remote: MongoProvider,
}

impl SchemaSyncService {
  /// Create new SchemaSyncService
  /// - mongo_uri: MongoDB connection URI (e.g., mongodb://localhost:27017)
  /// - db_name: MongoDB database name (e.g., schemas)
  pub async fn new(mongo_uri: &str, db_name: &str) -> OrmResult<Self> {
    let remote = MongoProvider::connect(mongo_uri, db_name).await?;
    Ok(Self { remote })
  }

  /// Sync schema from MongoDB into JsonProvider.
  /// Fetches from MongoDB, upserts into local JsonProvider, returns the schema.
  pub async fn sync_schema(
    &self,
    schema_id: &str,
    db: &JsonProvider,
  ) -> OrmResult<serde_json::Value> {
    let schema = self
      .remote
      .find_by_id("schemas", schema_id)
      .await?
      .ok_or_else(|| OrmError::NotFound(format!("Schema {} not found in MongoDB", schema_id)))?;

    let mut doc = schema;
    if doc
      .get("id")
      .and_then(|v| v.as_str())
      .is_none_or(|s| s.is_empty())
    {
      doc["id"] = serde_json::json!(schema_id);
    }

    let id = doc
      .get("id")
      .and_then(|v| v.as_str())
      .unwrap_or(schema_id)
      .to_string();

    if db.find_by_id("schemas", &id).await?.is_some() {
      db.update("schemas", &id, doc.clone()).await?;
      log_info!(
        "Schema '{}' synced from MongoDB and updated in JsonProvider",
        id
      );
    } else {
      db.insert("schemas", doc.clone()).await?;
      log_info!(
        "Schema '{}' synced from MongoDB and inserted into JsonProvider",
        id
      );
    }

    Ok(doc)
  }
}
