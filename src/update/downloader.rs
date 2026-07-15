use crate::update::models::DownloadProgress;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
  Client::builder()
    .pool_max_idle_per_host(8)
    .tcp_keepalive(std::time::Duration::from_secs(60))
    .build()
    .expect("reqwest Client build")
});

pub fn get_temp_download_path(asset_name: &str, app_name: &str) -> Result<PathBuf, String> {
  let temp_dir = std::env::temp_dir();
  let safe_name = asset_name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
  Ok(temp_dir.join(format!("{}_update_{}", app_name.to_lowercase(), safe_name)))
}

pub async fn download_update(
  url: &str,
  dest_path: &PathBuf,
  app_handle: &AppHandle,
  app_name: &str,
) -> Result<u64, String> {
  let response = HTTP_CLIENT
    .get(url)
    .header("Accept", "application/octet-stream")
    .header("User-Agent", app_name)
    .send()
    .await
    .map_err(|e| format!("Failed to start download: {}", e))?;

  if !response.status().is_success() {
    return Err(format!(
      "Download failed with status: {}",
      response.status()
    ));
  }

  let total_bytes = response.content_length().unwrap_or(0);
  let mut dest_file =
    std::fs::File::create(dest_path).map_err(|e| format!("Failed to create file: {}", e))?;

  let mut bytes_downloaded: u64 = 0;
  let mut stream = response.bytes_stream();
  let mut chunk_counter: u64 = 0;

  while let Some(chunk_result) = stream.next().await {
    match chunk_result {
      Ok(chunk) => {
        use std::io::Write;
        dest_file
          .write_all(&chunk)
          .map_err(|e| format!("Failed to write chunk: {}", e))?;
        bytes_downloaded += chunk.len() as u64;
        chunk_counter += 1;

        if chunk_counter.is_multiple_of(100) {
          let progress = if total_bytes > 0 {
            (bytes_downloaded as f64 / total_bytes as f64) * 100.0
          } else {
            0.0
          };
          let _ = app_handle.emit(
            "update-download-progress",
            DownloadProgress {
              bytes_downloaded,
              total_bytes,
              progress_percent: progress,
            },
          );
        }
      }
      Err(_e) => {
        break;
      }
    }
  }

  let _ = app_handle.emit(
    "update-download-complete",
    &dest_path.to_string_lossy().to_string(),
  );

  Ok(bytes_downloaded)
}
