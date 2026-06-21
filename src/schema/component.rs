use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
    pub slots: Vec<String>,
    #[serde(default)]
    pub allowed_children: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub i18n: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComponentProp {
    #[serde(default = "default_prop_type")]
    pub prop_type: String,
    #[serde(default)]
    pub default_value: Option<serde_json::Value>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub required: bool,
}

fn default_prop_type() -> String {
    "string".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEvent {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}
