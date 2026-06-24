use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TailwindBreakpoints {
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TailwindGridArea {
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct TailwindGridPosition {
    #[serde(default = "default_col")]
    pub col: i32,
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

fn default_col() -> i32 {
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TailwindResponsiveClasses {
    #[serde(default)]
    pub sm: Option<String>,
    #[serde(default)]
    pub md: Option<String>,
    #[serde(default)]
    pub lg: Option<String>,
    #[serde(default)]
    pub xl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TailwindGridElement {
    pub id: String,
    pub component_id: String,
    pub props: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub classes: Option<String>,
    #[serde(default)]
    pub grid_position: TailwindGridPosition,
    #[serde(default)]
    pub responsive_classes: Option<TailwindResponsiveClasses>,
    #[serde(default)]
    pub slot: Option<String>,
}
