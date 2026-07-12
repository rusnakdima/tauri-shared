use crate::update::{check_for_update, download_update, get_temp_download_path, install_update};
use tauri::AppHandle;

#[tauri::command(rename_all = "camelCase")]
pub async fn check_for_update_command(
    owner: String,
    repo: String,
    current_version: String,
    app_name: String,
) -> Result<crate::update::CheckUpdateResult, String> {
    match check_for_update(&owner, &repo, &current_version, &app_name).await {
        Ok(update_info) => Ok(crate::update::CheckUpdateResult {
            has_update: true,
            update_info: Some(update_info),
            error: None,
        }),
        Err(e) => {
            if e.contains("You are running the latest version") {
                Ok(crate::update::CheckUpdateResult {
                    has_update: false,
                    update_info: None,
                    error: None,
                })
            } else {
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
    let dest_path = get_temp_download_path(&asset_name, &app_name)?;
    let _downloaded = download_update(&url, &dest_path, &app_handle, &app_name).await?;
    Ok(dest_path.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn install_update_command(
    installer_path: String,
    app_handle: AppHandle,
) -> Result<bool, String> {
    install_update(&installer_path, &app_handle)
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
