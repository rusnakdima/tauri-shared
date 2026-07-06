use crate::schema::UiSchema;
use crate::AppError;
use crate::Result;
use nosql_orm::prelude::DatabaseProvider;
use nosql_orm::providers::json::JsonProvider;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SchemaSyncService {
    provider: Arc<Mutex<Option<JsonProvider>>>,
    data_dir: std::path::PathBuf,
}

impl SchemaSyncService {
    pub fn new(data_dir: &std::path::Path) -> Result<Self> {
        let schema_path = data_dir.join("schemas");
        std::fs::create_dir_all(&schema_path).map_err(|e| AppError::Io(e.to_string()))?;
        Ok(Self {
            provider: Arc::new(Mutex::new(None)),
            data_dir: schema_path,
        })
    }

    async fn get_provider(&self) -> Result<JsonProvider> {
        let mut guard = self.provider.lock().await;
        if guard.is_none() {
            let provider = JsonProvider::new(&self.data_dir)
                .await
                .map_err(AppError::from)?;
            *guard = Some(provider);
        }
        Ok(guard.clone().unwrap())
    }

    pub async fn get_schema_local(&self, app_id: &str) -> Result<Option<UiSchema>> {
        let provider = self.get_provider().await?;
        let data = provider
            .find_by_id("ui_schemas", app_id)
            .await
            .map_err(AppError::from)?;
        match data {
            Some(value) => {
                let schema: UiSchema = serde_json::from_value(value)
                    .map_err(|e| AppError::ValidationError(e.to_string()))?;
                Ok(Some(schema))
            }
            None => Ok(None),
        }
    }

    pub async fn save_schema_local(&self, app_id: &str, schema: &UiSchema) -> Result<()> {
        let provider = self.get_provider().await?;
        let value =
            serde_json::to_value(schema).map_err(|e| AppError::ValidationError(e.to_string()))?;
        if provider
            .find_by_id("ui_schemas", app_id)
            .await
            .map_err(AppError::from)?
            .is_some()
        {
            provider
                .update("ui_schemas", app_id, value)
                .await
                .map_err(AppError::from)?;
        } else {
            provider
                .insert("ui_schemas", value)
                .await
                .map_err(AppError::from)?;
        }
        Ok(())
    }
}
