use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub commands: Vec<CommandDef>,
    #[serde(default)]
    pub middleware: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandDef {
    pub name: String,
    #[serde(default)]
    pub params: Vec<String>,
    #[serde(default = "default_return_type")]
    pub return_type: String,
}

fn default_return_type() -> String {
    "void".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MiddlewareDef {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}
