use anyhow::{Result, anyhow};
use std::path::PathBuf;
use tokio::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn ensure_hls_for_media(db: &Arc<Mutex<rusqlite::Connection>>, media_id: &str, _quality: Option<String>) -> Result<String> {
  // locate a file path for media_id
  let conn = db.lock().await;
  let mut st = conn.prepare("SELECT path FROM files WHERE media_id=? LIMIT 1")?;
  let mut rows = st.query([media_id])?;
  let file_path: String = rows.next()?.ok_or_else(|| anyhow!("file not found"))?.get(0)?;
  drop(rows); drop(st); drop(conn);

  let out_dir = tauri::api::path::app_data_dir(&tauri::Config::default())
    .unwrap().join("hls").join(media_id);
  std::fs::create_dir_all(&out_dir)?;
  let m3u8 = out_dir.join("index.m3u8");
  if m3u8.exists() {
    // Already prepared; return relative path served by HTTP server
    return Ok(format!("hls/{}/index.m3u8", media_id));
  }

  // Start a lightweight HLS generation (copy when possible)
  let mut args = vec![
    "-i", &file_path,
    "-map", "0:v:0", "-map", "0:a:0?",
    "-codec:v", "libx264",
    "-codec:a", "aac",
    "-preset", "veryfast",
    "-start_number", "0",
    "-hls_time", "4", "-hls_list_size", "8",
    "-hls_flags", "delete_segments+independent_segments",
    "-f", "hls", "index.m3u8"
  ];
  // TODO: decide direct-play/remux vs transcode based on codecs

  let status = Command::new("ffmpeg")
    .args(&args)
    .current_dir(&out_dir)
    .status().await?;

  if !status.success() {
    return Err(anyhow!("ffmpeg failed"));
  }

  Ok(format!("hls/{}/index.m3u8", media_id))
}