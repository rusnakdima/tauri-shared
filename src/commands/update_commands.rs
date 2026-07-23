use crate::log_error;
use crate::log_info;
use crate::update::{check_for_update, download_update, get_temp_download_path, install_update};
use tauri::AppHandle;

#[tauri::command(rename_all = "camelCase")]
pub async fn check_for_update_command(
  owner: String,
  repo: String,
  current_version: String,
  app_name: String,
) -> Result<crate::update::CheckUpdateResult, String> {
  log_info!(
    "[BACKEND] CMD:check_for_update START owner={} repo={}",
    owner,
    repo
  );
  let start = std::time::Instant::now();
  match check_for_update(&owner, &repo, &current_version, &app_name).await {
    Ok(update_info) => {
      log_info!("[BACKEND] CMD:check_for_update OK ({:?})", start.elapsed());
      Ok(crate::update::CheckUpdateResult {
        has_update: true,
        update_info: Some(update_info),
        error: None,
      })
    }
    Err(e) => {
      if e.contains("You are running the latest version") {
        log_info!(
          "[BACKEND] CMD:check_for_update OK ({:?}) - no update",
          start.elapsed()
        );
        Ok(crate::update::CheckUpdateResult {
          has_update: false,
          update_info: None,
          error: None,
        })
      } else {
        log_error!(
          "[BACKEND] CMD:check_for_update ERROR ({:?}): {}",
          start.elapsed(),
          e
        );
        Ok(crate::update::CheckUpdateResult {
          has_update: false,
          update_info: None,
          error: Some(e),
        })
      }
    }
  }
}

#[tauri::command(rename_all = "camelCase")]
pub async fn download_update_command(
  url: String,
  asset_name: String,
  app_handle: AppHandle,
  app_name: String,
) -> Result<String, String> {
  log_info!("[BACKEND] CMD:download_update START asset={}", asset_name);
  let start = std::time::Instant::now();
  let result = get_temp_download_path(&asset_name, &app_name);
  let dest_path = match result {
    Ok(path) => path,
    Err(e) => {
      log_error!(
        "[BACKEND] CMD:download_update ERROR ({:?}): {}",
        start.elapsed(),
        e
      );
      return Err(e);
    }
  };
  match download_update(&url, &dest_path, &app_handle, &app_name).await {
    Ok(_) => {
      log_info!("[BACKEND] CMD:download_update OK ({:?})", start.elapsed());
      Ok(dest_path.to_string_lossy().to_string())
    }
    Err(e) => {
      log_error!(
        "[BACKEND] CMD:download_update ERROR ({:?}): {}",
        start.elapsed(),
        e
      );
      Err(e)
    }
  }
}

#[tauri::command(rename_all = "camelCase")]
pub async fn install_update_command(
  installer_path: String,
  app_handle: AppHandle,
) -> Result<bool, String> {
  log_info!("[BACKEND] CMD:install_update START");
  let start = std::time::Instant::now();
  let result = install_update(&installer_path, &app_handle);
  match &result {
    Ok(_) => log_info!("[BACKEND] CMD:install_update OK ({:?})", start.elapsed()),
    Err(e) => log_error!(
      "[BACKEND] CMD:install_update ERROR ({:?}): {}",
      start.elapsed(),
      e
    ),
  }
  result
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_current_version() -> String {
  log_info!("[BACKEND] CMD:get_current_version START");
  let version = env!("CARGO_PKG_VERSION").to_string();
  log_info!("[BACKEND] CMD:get_current_version OK version={}", version);
  version
}
