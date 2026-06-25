use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComponentDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub component_type: String,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub props: HashMap<String, ComponentProp>,
    #[serde(default)]
    pub default_classes: String,
    #[serde(default)]
    pub grid_defaults: Option<GridDefaults>,
    #[serde(default)]
    pub slots: Vec<String>,
    #[serde(default)]
    pub allowed_children: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub i18n: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GridDefaults {
    #[serde(default = "default_col_span")]
    pub default_col_span: i32,
    #[serde(default = "default_row_span")]
    pub default_row_span: i32,
    #[serde(default)]
    pub min_width: Option<String>,
    #[serde(default)]
    pub max_width: Option<String>,
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
pub struct ComponentProp {
    #[serde(default = "default_prop_type")]
    pub prop_type: String,
    #[serde(default)]
    #[ts(skip)]
    pub default_value: Option<serde_json::Value>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub required: bool,
}

fn default_prop_type() -> String {
    "string".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ComponentEvent {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}
