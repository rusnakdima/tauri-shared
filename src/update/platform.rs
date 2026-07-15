#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_platform_current_returns_valid() {
    let platform = Platform::current();
    // Just verify it returns a valid platform variant
    assert!(matches!(
      platform,
      Platform::Windows | Platform::Linux | Platform::MacOs | Platform::Android
    ));
  }

  #[test]
  fn test_platform_asset_extensions_windows() {
    let platform = Platform::Windows;
    let extensions = platform.asset_extensions();
    assert!(extensions.contains(&"msi"));
    assert!(extensions.contains(&"exe"));
  }

  #[test]
  fn test_platform_asset_extensions_macos() {
    let platform = Platform::MacOs;
    let extensions = platform.asset_extensions();
    assert!(extensions.contains(&"dmg"));
    assert!(extensions.contains(&"app.tar.gz"));
  }

  #[test]
  fn test_platform_asset_extensions_linux() {
    let platform = Platform::Linux;
    let extensions = platform.asset_extensions();
    assert!(extensions.contains(&"AppImage"));
    assert!(extensions.contains(&"deb"));
    assert!(extensions.contains(&"rpm"));
  }

  #[test]
  fn test_platform_asset_extensions_android() {
    let platform = Platform::Android;
    let extensions = platform.asset_extensions();
    assert!(extensions.contains(&"apk"));
  }

  #[test]
  fn test_platform_asset_name_prefix() {
    assert_eq!(Platform::Windows.asset_name_prefix("MyApp"), "myapp");
    assert_eq!(Platform::Linux.asset_name_prefix("MyApp"), "myapp");
    assert_eq!(Platform::MacOs.asset_name_prefix("MyApp"), "myapp");
    assert_eq!(Platform::Android.asset_name_prefix("MyApp"), "myapp");
  }

  #[test]
  fn test_platform_asset_name_with_version_windows() {
    let platform = Platform::Windows;
    let name = platform.asset_name_with_version("MyApp", "1.0.0");
    assert_eq!(name, "myapp_1.0.0_windows");
  }

  #[test]
  fn test_platform_asset_name_with_version_linux() {
    let platform = Platform::Linux;
    let name = platform.asset_name_with_version("MyApp", "1.0.0");
    assert_eq!(name, "myapp_1.0.0_linux");
  }

  #[test]
  fn test_platform_asset_name_with_version_macos() {
    let platform = Platform::MacOs;
    let name = platform.asset_name_with_version("MyApp", "1.0.0");
    assert_eq!(name, "myapp_1.0.0_macos");
  }

  #[test]
  fn test_platform_asset_name_with_version_android() {
    let platform = Platform::Android;
    let name = platform.asset_name_with_version("MyApp", "1.0.0");
    assert_eq!(name, "myapp_1.0.0_android");
  }

  #[test]
  fn test_has_matching_extension_true() {
    let extensions = vec!["msi", "exe"];
    assert!(has_matching_extension("myapp.msi", &extensions));
    assert!(has_matching_extension("myapp.exe", &extensions));
    assert!(has_matching_extension("setup.msi", &extensions));
  }

  #[test]
  fn test_has_matching_extension_false() {
    let extensions = vec!["msi", "exe"];
    assert!(!has_matching_extension("myapp.tar.gz", &extensions));
    assert!(!has_matching_extension("myapp.deb", &extensions));
    assert!(!has_matching_extension("noextension", &extensions));
  }

  #[test]
  fn test_has_matching_extension_case_sensitive() {
    let extensions = vec!["msi", "exe"];
    // Extension matching is case-sensitive
    assert!(has_matching_extension("myapp.msi", &extensions));
    assert!(has_matching_extension("myapp.exe", &extensions));
    assert!(!has_matching_extension("myapp.MSI", &extensions));
    assert!(!has_matching_extension("myapp.Exe", &extensions));
  }

  #[test]
  fn test_platform_display() {
    assert_eq!(format!("{}", Platform::Windows), "Windows");
    assert_eq!(format!("{}", Platform::Linux), "Linux");
    assert_eq!(format!("{}", Platform::MacOs), "macOS");
    assert_eq!(format!("{}", Platform::Android), "Android");
  }

  #[test]
  fn test_platform_debug() {
    let platform = Platform::Windows;
    let debug_str = format!("{:?}", platform);
    assert!(debug_str.contains("Windows"));
  }

  #[test]
  fn test_platform_clone() {
    let platform = Platform::current();
    let cloned = platform;
    assert_eq!(platform, cloned);
  }

  #[test]
  fn test_platform_copy() {
    let platform = Platform::Windows;
    let copied = platform;
    assert_eq!(platform, copied);
  }

  #[test]
  fn test_platform_serialization() {
    let platform = Platform::Windows;
    let json = serde_json::to_string(&platform).unwrap();
    assert!(json.contains("Windows") || json.contains("windows"));
  }
}

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

  pub fn asset_name_with_version(&self, app_name: &str, version: &str) -> String {
    let platform_str = match self {
      Platform::Windows => "windows",
      Platform::Linux => "linux",
      Platform::MacOs => "macos",
      Platform::Android => "android",
    };
    format!("{}_{}_{}", app_name.to_lowercase(), version, platform_str)
  }
}

pub fn has_matching_extension(name: &str, extensions: &[&str]) -> bool {
  extensions
    .iter()
    .any(|ext| name.ends_with(&format!(".{}", ext)))
}
