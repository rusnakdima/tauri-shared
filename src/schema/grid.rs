use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use super::page::GridPosition;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ResponsiveBreakpoints {
  #[serde(default = "default_mobile_cols")]
  pub mobile_cols: i32,
  #[serde(default)]
  pub sm_cols: Option<i32>,
  #[serde(default)]
  pub md_cols: Option<i32>,
  #[serde(default)]
  pub lg_cols: Option<i32>,
  #[serde(default)]
  pub xl_cols: Option<i32>,
  #[serde(default = "default_gap")]
  pub gap: String,
  #[serde(default)]
  pub sm_gap: Option<String>,
  #[serde(default)]
  pub md_gap: Option<String>,
  #[serde(default)]
  pub lg_gap: Option<String>,
  #[serde(default)]
  pub xl_gap: Option<String>,
}

fn default_mobile_cols() -> i32 {
  1
}

fn default_gap() -> String {
  "16px".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NamedGridArea {
  pub name: String,
  #[serde(default)]
  pub col_start: Option<i32>,
  #[serde(default)]
  pub col_end: Option<i32>,
  #[serde(default)]
  pub row_start: Option<i32>,
  #[serde(default)]
  pub row_end: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ResponsiveClasses {
  #[serde(default)]
  pub sm: Option<String>,
  #[serde(default)]
  pub md: Option<String>,
  #[serde(default)]
  pub lg: Option<String>,
  #[serde(default)]
  pub xl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GridElement {
  pub id: String,
  pub component_id: String,
  #[ts(skip)]
  pub props: HashMap<String, serde_json::Value>,
  #[serde(default)]
  pub classes: Option<String>,
  #[serde(default)]
  pub grid_position: GridPosition,
  #[serde(default)]
  pub responsive_classes: Option<ResponsiveClasses>,
  #[serde(default)]
  pub slot: Option<String>,
}
