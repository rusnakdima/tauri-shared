use super::theme::{get_dark_theme, get_light_theme, Theme};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppConfig {
  pub id: String,
  pub name: String,
  pub version: String,
  #[serde(default)]
  pub description: String,
  pub identifier: String,
  pub settings: AppSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppSettings {
  pub default_locale: String,
  pub supported_locales: Vec<String>,
  #[serde(default = "default_tailwind_preset")]
  pub tailwind_preset: String,
  #[serde(default = "default_theme_name")]
  pub theme: String,
  #[serde(default)]
  pub themes: Vec<Theme>,
  #[serde(default = "default_color_mode")]
  pub color_mode: ColorMode,
}

fn default_tailwind_preset() -> String {
  "default".to_string()
}

fn default_theme_name() -> String {
  "light".to_string()
}

fn default_color_mode() -> ColorMode {
  ColorMode::System
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum ColorMode {
  Light,
  Dark,
  System,
}

impl Default for AppSettings {
  fn default() -> Self {
    Self {
      default_locale: "en".to_string(),
      supported_locales: vec!["en".to_string()],
      tailwind_preset: default_tailwind_preset(),
      theme: default_theme_name(),
      themes: vec![get_light_theme(), get_dark_theme()],
      color_mode: default_color_mode(),
    }
  }
}
