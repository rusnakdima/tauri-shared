use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub name: String,
    #[serde(default)]
    pub label: String,
    pub colors: ThemeColors,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColors {
    #[serde(default = "default_bg_primary")]
    pub bg_primary: String,
    #[serde(default = "default_bg_secondary")]
    pub bg_secondary: String,
    #[serde(default = "default_bg_tertiary")]
    pub bg_tertiary: String,
    #[serde(default = "default_text_primary")]
    pub text_primary: String,
    #[serde(default = "default_text_secondary")]
    pub text_secondary: String,
    #[serde(default = "default_text_muted")]
    pub text_muted: String,
    #[serde(default = "default_border")]
    pub border: String,
    #[serde(default = "default_primary")]
    pub primary: String,
    #[serde(default = "default_secondary")]
    pub secondary: String,
    #[serde(default = "default_accent")]
    pub accent: String,
    #[serde(default = "default_success")]
    pub success: String,
    #[serde(default = "default_warning")]
    pub warning: String,
    #[serde(default = "default_error")]
    pub error: String,
}

fn default_bg_primary() -> String { "#ffffff".to_string() }
fn default_bg_secondary() -> String { "#f3f4f6".to_string() }
fn default_bg_tertiary() -> String { "#e5e7eb".to_string() }
fn default_text_primary() -> String { "#111827".to_string() }
fn default_text_secondary() -> String { "#374151".to_string() }
fn default_text_muted() -> String { "#6b7280".to_string() }
fn default_border() -> String { "#d1d5db".to_string() }
fn default_primary() -> String { "#3b82f6".to_string() }
fn default_secondary() -> String { "#6b7280".to_string() }
fn default_accent() -> String { "#10b981".to_string() }
fn default_success() -> String { "#22c55e".to_string() }
fn default_warning() -> String { "#f59e0b".to_string() }
fn default_error() -> String { "#ef4444".to_string() }

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_primary: default_bg_primary(),
            bg_secondary: default_bg_secondary(),
            bg_tertiary: default_bg_tertiary(),
            text_primary: default_text_primary(),
            text_secondary: default_text_secondary(),
            text_muted: default_text_muted(),
            border: default_border(),
            primary: default_primary(),
            secondary: default_secondary(),
            accent: default_accent(),
            success: default_success(),
            warning: default_warning(),
            error: default_error(),
        }
    }
}

pub fn get_light_theme() -> Theme {
    Theme {
        name: "light".to_string(),
        label: "Light".to_string(),
        colors: ThemeColors::default(),
    }
}

pub fn get_dark_theme() -> Theme {
    Theme {
        name: "dark".to_string(),
        label: "Dark".to_string(),
        colors: ThemeColors {
            bg_primary: "#0f172a".to_string(),
            bg_secondary: "#1e293b".to_string(),
            bg_tertiary: "#334155".to_string(),
            text_primary: "#f1f5f9".to_string(),
            text_secondary: "#cbd5e1".to_string(),
            text_muted: "#94a3b8".to_string(),
            border: "#475569".to_string(),
            primary: "#3b82f6".to_string(),
            secondary: "#64748b".to_string(),
            accent: "#10b981".to_string(),
            success: "#22c55e".to_string(),
            warning: "#f59e0b".to_string(),
            error: "#ef4444".to_string(),
        },
    }
}
