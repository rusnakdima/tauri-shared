use crate::schema::UiSchema;
use crate::storage::json_db::JsonDb;

pub struct SchemaSyncService {
    local_db: JsonDb,
}

impl SchemaSyncService {
    pub fn new(data_dir: &std::path::Path) -> Result<Self, String> {
        let schema_path = data_dir.join("schemas");
        std::fs::create_dir_all(&schema_path)
            .map_err(|e| format!("Failed to create schema dir: {}", e))?;
        let local_db =
            JsonDb::new(&schema_path).map_err(|e| format!("Failed to init local db: {}", e))?;
        Ok(Self { local_db })
    }

    pub fn get_schema_local(&self, app_id: &str) -> Result<Option<UiSchema>, String> {
        let collection = format!("{}_schemas", app_id);
        let path = self.local_db.get_collection_path(&collection);
        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read schema: {}", e))?;
            let schema: UiSchema = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse schema: {}", e))?;
            Ok(Some(schema))
        } else {
            Ok(None)
        }
    }

    pub fn save_schema_local(&self, app_id: &str, schema: &UiSchema) -> Result<(), String> {
        let collection = format!("{}_schemas", app_id);
        let content = serde_json::to_string_pretty(schema)
            .map_err(|e| format!("Failed to serialize schema: {}", e))?;
        let path = self.local_db.get_collection_path(&collection);
        std::fs::write(&path, content).map_err(|e| format!("Failed to write schema: {}", e))?;
        Ok(())
    }
}
