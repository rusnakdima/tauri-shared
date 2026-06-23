use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub slots: HashMap<String, LayoutSlot>,
    #[serde(default)]
    pub grid_template: Option<GridTemplate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LayoutSlot {
    #[serde(default)]
    pub component_id: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
    #[serde(default)]
    pub width: Option<String>,
    #[serde(default)]
    pub dynamic: bool,
    #[serde(default)]
    pub grid_column: Option<String>,
    #[serde(default)]
    pub grid_row: Option<String>,
    #[serde(default)]
    pub grid_area: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GridTemplate {
    pub id: String,
    #[serde(default = "default_grid_columns")]
    pub columns: Vec<GridTrack>,
    #[serde(default = "default_grid_rows")]
    pub rows: Vec<GridTrack>,
    #[serde(default = "default_gap")]
    pub gap: String,
    #[serde(default)]
    pub areas: Option<Vec<GridArea>>,
}

fn default_grid_columns() -> Vec<GridTrack> {
    vec![GridTrack {
        size: "1fr".to_string(),
        min: None,
        max: None,
    }]
}

fn default_grid_rows() -> Vec<GridTrack> {
    vec![GridTrack {
        size: "auto".to_string(),
        min: None,
        max: None,
    }]
}

fn default_gap() -> String {
    "16px".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GridTrack {
    pub size: String,
    #[serde(default)]
    pub min: Option<String>,
    #[serde(default)]
    pub max: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GridArea {
    pub name: String,
    pub column_start: i32,
    pub column_end: i32,
    pub row_start: i32,
    pub row_end: i32,
}
