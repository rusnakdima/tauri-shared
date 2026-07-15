#[cfg(test)]
mod tests {
  use super::*;
  use crate::update::models::GitHubAsset;

  #[test]
  fn test_is_newer_version_newer() {
    assert!(is_newer_version("1.0.0", "1.0.1"));
    assert!(is_newer_version("1.0.0", "1.1.0"));
    assert!(is_newer_version("1.0.0", "2.0.0"));
    assert!(is_newer_version("1.0.0", "1.0.0.1"));
  }

  #[test]
  fn test_is_newer_version_same() {
    assert!(!is_newer_version("1.0.0", "1.0.0"));
    assert!(!is_newer_version("2.0.0", "2.0.0"));
  }

  #[test]
  fn test_is_newer_version_older() {
    assert!(!is_newer_version("1.0.1", "1.0.0"));
    assert!(!is_newer_version("2.0.0", "1.0.0"));
  }

  #[test]
  fn test_is_newer_version_with_v_prefix() {
    assert!(is_newer_version("v1.0.0", "v1.0.1"));
    assert!(is_newer_version("1.0.0", "v1.0.1"));
    assert!(is_newer_version("v1.0.0", "1.0.1"));
  }

  #[test]
  fn test_is_newer_version_uneven_parts() {
    // 1.0 vs 1.0.1 - 1.0.1 is newer
    assert!(is_newer_version("1.0", "1.0.1"));
    // "1" vs "1.0.0" - same version, different representations
    assert!(!is_newer_version("1", "1.0.0"));
  }

  #[test]
  fn test_find_platform_asset_windows() {
    let platform = Platform::Windows;
    let assets = vec![GitHubAsset {
      id: 1,
      name: "myapp_1.0.0_windows.msi".to_string(),
      browser_download_url: "https://example.com/myapp_1.0.0_windows.msi".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "myapp_1.0.0_windows.msi");
  }

  #[test]
  fn test_find_platform_asset_linux() {
    let platform = Platform::Linux;
    let assets = vec![GitHubAsset {
      id: 1,
      name: "myapp_1.0.0_linux.AppImage".to_string(),
      browser_download_url: "https://example.com/myapp_1.0.0_linux.AppImage".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_some());
  }

  #[test]
  fn test_find_platform_asset_android_universal() {
    let platform = Platform::Android;
    let assets = vec![GitHubAsset {
      id: 1,
      name: "app-universal-release.apk".to_string(),
      browser_download_url: "https://example.com/app-universal-release.apk".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_some());
    assert_eq!(result.unwrap().name, "app-universal-release.apk");
  }

  #[test]
  fn test_find_platform_asset_no_match() {
    let platform = Platform::Windows;
    // Use .zip extension which is not in the Windows extensions list
    let assets = vec![GitHubAsset {
      id: 1,
      name: "otherapp_1.0.0_windows.zip".to_string(),
      browser_download_url: "https://example.com/otherapp_1.0.0_windows.zip".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    // otherapp doesn't match myapp prefix, and .zip is not a valid windows extension
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_none());
  }

  #[test]
  fn test_find_platform_asset_empty_assets() {
    let platform = Platform::Windows;
    let assets: Vec<GitHubAsset> = vec![];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_none());
  }

  #[test]
  fn test_find_platform_asset_case_insensitive() {
    let platform = Platform::Windows;
    let assets = vec![GitHubAsset {
      id: 1,
      name: "MYAPP_1.0.0_WINDOWS.MSI".to_string(),
      browser_download_url: "https://example.com/MYAPP_1.0.0_WINDOWS.MSI".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_some());
  }

  #[test]
  fn test_find_platform_asset_macos_dmg() {
    let platform = Platform::MacOs;
    let assets = vec![GitHubAsset {
      id: 1,
      name: "myapp_1.0.0_macos.dmg".to_string(),
      browser_download_url: "https://example.com/myapp_1.0.0_macos.dmg".to_string(),
      size: 100,
      content_type: "application/octet-stream".to_string(),
    }];
    let result = find_platform_asset(&assets, &platform, "myapp", "1.0.0");
    assert!(result.is_some());
  }
}

use crate::update::models::{GitHubAsset, GitHubRelease, UpdateInfo};
use crate::update::platform::{has_matching_extension, Platform};
use once_cell::sync::Lazy;
use reqwest::Client;
use std::cmp::Ordering;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
  Client::builder()
    .pool_max_idle_per_host(8)
    .tcp_keepalive(std::time::Duration::from_secs(60))
    .build()
    .expect("reqwest Client build")
});

fn compare_versions(current: &str, latest: &str) -> Ordering {
  let current = current.trim_start_matches('v');
  let latest = latest.trim_start_matches('v');

  let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
  let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();

  let max_len = current_parts.len().max(latest_parts.len());
  for i in 0..max_len {
    let c = current_parts.get(i).unwrap_or(&0);
    let l = latest_parts.get(i).unwrap_or(&0);
    match c.cmp(l) {
      Ordering::Equal => continue,
      other => return other,
    }
  }
  Ordering::Equal
}

pub fn is_newer_version(current: &str, latest: &str) -> bool {
  compare_versions(current, latest) == Ordering::Less
}

pub fn find_platform_asset<'a>(
  assets: &'a [GitHubAsset],
  platform: &Platform,
  app_name: &str,
  version: &str,
) -> Option<&'a GitHubAsset> {
  let extensions = platform.asset_extensions();
  let prefix = platform.asset_name_with_version(app_name, version);

  // First try exact prefix match with extension
  for asset in assets {
    let name_lower = asset.name.to_lowercase();
    if name_lower.contains(&prefix) && has_matching_extension(&name_lower, &extensions) {
      return Some(asset);
    }
  }

  // Fallback: just prefix match
  for asset in assets {
    let name_lower = asset.name.to_lowercase();
    if name_lower.contains(&prefix) {
      return Some(asset);
    }
  }

  // Android universal fallback
  if matches!(platform, Platform::Android) {
    for asset in assets {
      let name_lower = asset.name.to_lowercase();
      if name_lower.contains("app-universal-release")
        && has_matching_extension(&name_lower, &extensions)
      {
        return Some(asset);
      }
    }
  }

  // Last resort: just extension match
  for asset in assets {
    let name_lower = asset.name.to_lowercase();
    if has_matching_extension(&name_lower, &extensions) {
      return Some(asset);
    }
  }

  None
}

pub async fn check_for_update(
  owner: &str,
  repo: &str,
  current_version: &str,
  app_name: &str,
) -> Result<UpdateInfo, String> {
  let url = format!(
    "https://api.github.com/repos/{}/{}/releases/latest",
    owner, repo
  );
  let response = HTTP_CLIENT
    .get(&url)
    .header("Accept", "application/vnd.github+json")
    .header("User-Agent", app_name)
    .send()
    .await
    .map_err(|e| format!("Failed to fetch release info: {}", e))?;

  if response.status() == 403 {
    return Err("GitHub API rate limit exceeded. Please try again later.".to_string());
  }
  if response.status() == 404 {
    return Err("No releases found.".to_string());
  }

  let release: GitHubRelease = response
    .json()
    .await
    .map_err(|e| format!("Failed to parse release info: {}", e))?;

  let latest_version = release.tag_name.trim_start_matches('v');

  if !is_newer_version(current_version, latest_version) {
    return Err("You are running the latest version.".to_string());
  }

  let platform = Platform::current();
  let asset = find_platform_asset(&release.assets, &platform, app_name, latest_version)
    .ok_or_else(|| format!("No suitable asset found for {:?}", platform))?;

  Ok(UpdateInfo {
    current_version: current_version.to_string(),
    latest_version: latest_version.to_string(),
    download_url: asset.browser_download_url.clone(),
    asset_name: asset.name.clone(),
    asset_size: asset.size,
    release_notes: release.body,
  })
}
