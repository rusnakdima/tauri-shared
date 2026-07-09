use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Page {
  pub id: String,
  pub name: String,
  pub route: String,
  #[serde(default)]
  pub layout: Option<String>,
  #[serde(default)]
  pub meta: PageMeta,
  #[serde(default)]
  pub sections: HashMap<String, PageSection>,
  #[serde(default)]
  pub canvas_elements: Vec<CanvasElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PageMeta {
  #[serde(default)]
  pub title: Option<String>,
  #[serde(default)]
  pub icon: Option<String>,
  #[serde(default)]
  pub breadcrumb: Vec<String>,
}

impl Default for PageMeta {
  fn default() -> Self {
    Self {
      title: None,
      icon: None,
      breadcrumb: Vec::new(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PageSection {
  #[serde(default)]
  pub component_id: Option<String>,
  #[serde(default = "default_visible")]
  pub visible: bool,
  #[serde(default)]
  pub dynamic: bool,
}

fn default_visible() -> bool {
  true
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CanvasElement {
  pub id: String,
  pub component_id: String,
  #[serde(default)]
  pub grid_position: GridPosition,
  #[serde(default)]
  #[ts(skip)]
  pub props: HashMap<String, serde_json::Value>,
  #[serde(default)]
  pub classes: String,
  #[serde(default)]
  pub children: Vec<String>,
  #[serde(default)]
  pub data_binding: Option<DataBinding>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GridPosition {
  #[serde(default = "default_column")]
  pub column: i32,
  #[serde(default = "default_row")]
  pub row: i32,
  #[serde(default = "default_col_span")]
  pub col_span: i32,
  #[serde(default = "default_row_span")]
  pub row_span: i32,
  #[serde(default)]
  pub col_start: Option<i32>,
  #[serde(default)]
  pub row_start: Option<i32>,
}

impl Default for GridPosition {
  fn default() -> Self {
    Self {
      column: 1,
      row: 1,
      col_span: 1,
      row_span: 1,
      col_start: None,
      row_start: None,
    }
  }
}

fn default_column() -> i32 {
  1
}

fn default_row() -> i32 {
  1
}

fn default_col_span() -> i32 {
  1
}

fn default_row_span() -> i32 {
  1
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DataBinding {
  pub entity: String,
  #[serde(default)]
  pub field: Option<String>,
}
