use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ElementLayout {
  #[serde(default)]
  pub direction: Option<String>,
  #[serde(default)]
  pub main_axis: Option<String>,
  #[serde(default)]
  pub cross_axis: Option<String>,
  #[serde(default)]
  pub main_axis_size: Option<String>,
  #[serde(default)]
  pub spacing: Option<i32>,
  #[serde(default)]
  pub col_gap: Option<i32>,
  #[serde(default)]
  pub row_gap: Option<i32>,
  #[serde(default)]
  pub overflow: Option<String>,
  #[serde(default)]
  pub align: Option<String>,
  #[serde(default)]
  pub columns: Option<i32>,
}
