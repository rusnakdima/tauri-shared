use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
pub struct DataBinding {
  pub entity: String,
  #[serde(default)]
  pub field: Option<String>,
  #[serde(default)]
  pub store: Option<String>,
  #[serde(default)]
  pub transform: Option<String>,
}
