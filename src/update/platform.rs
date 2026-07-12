use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    Linux,
    MacOs,
    Android,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Windows => write!(f, "Windows"),
            Platform::Linux => write!(f, "Linux"),
            Platform::MacOs => write!(f, "macOS"),
            Platform::Android => write!(f, "Android"),
        }
    }
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Platform::Windows;
        #[cfg(target_os = "macos")]
        return Platform::MacOs;
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        #[cfg(target_os = "android")]
        return Platform::Android;
    }

    pub fn asset_extensions(&self) -> Vec<&'static str> {
        match self {
            Platform::Windows => vec!["msi", "exe"],
            Platform::MacOs => vec!["dmg", "app.tar.gz"],
            Platform::Linux => vec!["AppImage", "deb", "rpm"],
            Platform::Android => vec!["apk"],
        }
    }

    pub fn asset_name_prefix(&self, app_name: &str) -> String {
        app_name.to_lowercase()
    }
}

pub fn asset_name_prefix(app_name: &str, version: &str, platform: &Platform) -> String {
    let platform_str = match platform {
        Platform::Windows => "windows",
        Platform::Linux => "linux",
        Platform::MacOs => "macos",
        Platform::Android => "android",
    };
    format!("{}_{}_{}", app_name.to_lowercase(), version, platform_str)
}

pub fn has_matching_extension(name: &str, extensions: &[&str]) -> bool {
    extensions
        .iter()
        .any(|ext| name.ends_with(&format!(".{}", ext)))
}
