use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CanvasElement {
    pub id: String,
    pub component_id: String,
    #[serde(default)]
    pub position: ElementPosition,
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub classes: String,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default)]
    pub data_binding: Option<DataBinding>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ElementPosition {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default = "default_width")]
    pub width: i32,
    #[serde(default = "default_height")]
    pub height: i32,
}

impl Default for ElementPosition {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 400,
            height: 200,
        }
    }
}

fn default_width() -> i32 {
    400
}

fn default_height() -> i32 {
    200
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DataBinding {
    pub entity: String,
    #[serde(default)]
    pub field: Option<String>,
}
