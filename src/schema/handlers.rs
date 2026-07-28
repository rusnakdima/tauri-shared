use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct HandlerDef {
  #[serde(default)]
  pub handler_type: Option<String>,
  #[serde(default)]
  pub command: Option<String>,
  #[serde(default)]
  pub store: Option<String>,
  #[serde(default)]
  pub field: Option<String>,
  #[serde(default)]
  pub fn_name: Option<String>,
  #[serde(default)]
  #[ts(skip)]
  pub config: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StoreDef {
  pub id: String,
  #[serde(default)]
  #[ts(skip)]
  pub initial: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DataSourceDef {
  pub id: String,
  #[serde(default)]
  pub datasource_type: Option<String>,
  #[serde(default)]
  #[ts(skip)]
  pub config: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ActionDef {
  pub id: String,
  pub label: String,
  #[serde(default)]
  pub action_type: Option<String>,
  #[serde(default)]
  #[ts(skip)]
  pub config: HashMap<String, serde_json::Value>,
}
