pub mod app;
pub mod component;
pub mod grid;
pub mod i18n;
pub mod layout;
pub mod module;
pub mod page;
pub mod service;
pub mod theme;

pub use app::*;
pub use component::*;
pub use grid::*;
pub use i18n::*;
pub use layout::*;
pub use module::*;
pub use page::*;
pub use service::*;
pub use theme::*;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
