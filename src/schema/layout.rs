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
}
