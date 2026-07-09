use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::app::AppConfig;
use super::component::ComponentDef;
use super::i18n::I18nConfig;
use super::layout::Layout;
use super::module::ModuleDef;
use super::page::Page;
use super::service::ServiceDef;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UiSchema {
  pub schema_version: String,
  pub app: AppConfig,
  pub pages: Vec<Page>,
  pub layouts: Vec<Layout>,
  pub components: Vec<ComponentDef>,
  #[serde(default)]
  pub shared_components: Vec<ComponentDef>,
  pub services: Vec<ServiceDef>,
  pub modules: Vec<ModuleDef>,
  pub i18n: I18nConfig,
}
