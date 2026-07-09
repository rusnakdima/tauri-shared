use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct I18nConfig {
  #[serde(default)]
  pub locales: HashMap<String, LocaleMap>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LocaleMap {
  #[serde(default)]
  pub nav: HashMap<String, String>,
  #[serde(default)]
  pub actions: HashMap<String, String>,
  #[serde(default)]
  pub messages: HashMap<String, String>,
  #[serde(flatten)]
  #[ts(skip)]
  pub extra: HashMap<String, serde_json::Value>,
}
