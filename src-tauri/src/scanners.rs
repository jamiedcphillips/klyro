use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use tokio::process::Command;
use serde::Deserialize;
use crate::db;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
struct FFProbe {
  streams: Option<Vec<FFStream>>,
  format: Option<FFFormat>,
}
#[derive(Debug, Deserialize)]
struct FFStream {
  codec_type: Option<String>,
  codec_name: Option<String>,
  width: Option<i64>,
  height: Option<i64>,
}
#[derive(Debug, Deserialize)]
struct FFFormat {
  duration: Option<String>,
  size: Option<String>,
}

fn is_media_file(p: &Path) -> bool {
  if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
    matches!(ext.to_lowercase().as_str(), "mp4" | "mkv" | "mov" | "m4v" | "avi" | "mpg" | "ts")
  } else { false }
}

fn make_ids(path: &Path) -> (String, String, String) {
  let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
  // naive: treat everything as "movie" for MVP; we’ll enrich later.
  let media_type = "movie";
  let norm = file_stem.to_string();
  let media_id = format!("local:{}", blake3::hash(norm.as_bytes()).to_hex());
  let file_id = format!("file:{}", blake3::hash(path.to_string_lossy().as_bytes()).to_hex());
  (media_type.to_string(), media_id, file_id)
}

pub async fn scan_all(dbh: &Arc<Mutex<rusqlite::Connection>>) -> Result<u32> {
  let paths = crate::db::list_library_paths(dbh).map_err(|e| anyhow!(e))?;
  let mut count = 0u32;
  for p in paths {
    count += scan_path(dbh, Path::new(&p)).await?;
  }
  Ok(count)
}

async fn ffprobe(path: &Path) -> Result<FFProbe> {
  let out = Command::new("ffprobe")
    .arg("-v").arg("quiet")
    .arg("-print_format").arg("json")
    .arg("-show_format").arg("-show_streams")
    .arg(path)
    .output().await?;
  if !out.status.success() {
    return Err(anyhow!("ffprobe failed"));
  }
  let v: FFProbe = serde_json::from_slice(&out.stdout)?;
  Ok(v)
}

async fn scan_path(dbh: &Arc<Mutex<rusqlite::Connection>>, root: &Path) -> Result<u32> {
  let mut added = 0u32;
  for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
    let path = e.path();
    if path.is_file() && is_media_file(path) {
      let (media_type, media_id, file_id) = make_ids(path);
      let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown");
      let probe = ffprobe(path).await.ok();
      let (dur, w, h, vcodec, acodec, size) = if let Some(pr) = probe {
        let dur = pr.format.as_ref().and_then(|f| f.duration.as_deref()).and_then(|s| s.parse::<f64>().ok()).map(|d| d as i64);
        let size = pr.format.as_ref().and_then(|f| f.size.as_deref()).and_then(|s| s.parse::<i64>().ok());
        let mut w=None; let mut h=None; let mut v=None; let mut a=None;
        if let Some(streams) = pr.streams {
          for s in streams {
            match s.codec_type.as_deref() {
              Some("video") => { w=s.width; h=s.height; v=s.codec_name; }
              Some("audio") => { a=s.codec_name; }
              _ => {}
            }
          }
        }
        (dur, w, h, v.as_deref(), a.as_deref(), size)
      } else {(None,None,None,None,None,None)};
      db::upsert_local_media_and_file(
        dbh, &media_id, name, &media_type, &file_id, &path.to_string_lossy(),
        dur, w, h, vcodec, acodec, size
      )?;
      added += 1;
    }
  }
  Ok(added)
}