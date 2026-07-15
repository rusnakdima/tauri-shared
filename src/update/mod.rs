pub mod checker;
pub mod downloader;
pub mod installer;
pub mod models;
pub mod platform;

pub use checker::{check_for_update, find_platform_asset, is_newer_version};
pub use downloader::{download_update, get_temp_download_path};
pub use installer::install_update;
pub use models::{DownloadProgress, GitHubAsset, GitHubRelease, UpdateInfo};
pub use platform::Platform;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckUpdateResult {
  pub has_update: bool,
  pub update_info: Option<UpdateInfo>,
  pub error: Option<String>,
}
