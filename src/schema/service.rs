use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ServiceDef {
    pub id: String,
    pub name: String,
    pub entity: String,
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default)]
    pub crud: ServiceCrud,
    #[serde(default)]
    pub fields: Vec<ServiceField>,
}

fn default_provider() -> String {
    "nosql".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS, Default)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ServiceCrud {
    #[serde(default = "default_crud_true")]
    pub create: bool,
    #[serde(default = "default_crud_true")]
    pub read: bool,
    #[serde(default = "default_crud_true")]
    pub update: bool,
    #[serde(default = "default_crud_true")]
    pub delete: bool,
    #[serde(default = "default_crud_true")]
    pub query: bool,
}

fn default_crud_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ServiceField {
    pub name: String,
    #[serde(default = "default_field_type")]
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

fn default_field_type() -> String {
    "string".to_string()
}
