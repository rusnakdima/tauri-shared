use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct I18nConfig {
    #[serde(default)]
    pub locales: HashMap<String, LocaleMap>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LocaleMap {
    #[serde(default)]
    pub nav: HashMap<String, String>,
    #[serde(default)]
    pub actions: HashMap<String, String>,
    #[serde(default)]
    pub messages: HashMap<String, String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
