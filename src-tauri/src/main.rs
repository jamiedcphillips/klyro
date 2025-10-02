#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod db;
mod models;
mod scanner;
mod ffmpeg;
mod server;
mod addons;
mod iptv;
mod tasks;

use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

pub struct AppState {
  pub db: Arc<Mutex<rusqlite::Connection>>,
  pub http_port: u16,
}

#[tauri::command]
async fn add_library_path(state: State<'_, AppState>, path: String) -> Result<(), String> {
  db::add_library_path(&state.db, &path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_library_paths(state: State<'_, AppState>) -> Result<Vec<String>, String> {
  db::list_library_paths(&state.db).map_err(|e| e.to_string())
}

#[tauri::command]
async fn scan_library(state: State<'_, AppState>) -> Result<u32, String> {
  scanner::scan_all(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_continue_watching(state: State<'_, AppState>) -> Result<Vec<models::ContinueItem>, String> {
  db::get_continue_watching(&state.db).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_progress(state: State<'_, AppState>, media_id: String, position_sec: u64, duration_sec: u64) -> Result<(), String> {
  db::update_progress(&state.db, "default", &media_id, position_sec as i64, duration_sec as i64)
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_movies(state: State<'_, AppState>) -> Result<Vec<models::MediaItem>, String> {
  db::list_media_by_type(&state.db, "movie").map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_series(state: State<'_, AppState>) -> Result<Vec<models::MediaItem>, String> {
  db::list_media_by_type(&state.db, "series").map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_stream(state: State<'_, AppState>, media_id: String, quality: Option<String>) -> Result<String, String> {
  ffmpeg::ensure_hls_for_media(&state.db, &media_id, quality).await
    .map(|hls_path| format!("http://127.0.0.1:{}/{}", state.http_port, hls_path))
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_http_base(state: State<'_, AppState>) -> Result<String, String> {
  Ok(format!("http://127.0.0.1:{}", state.http_port))
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle();
      let app_dir = app_handle.path_resolver().app_data_dir().unwrap();
      std::fs::create_dir_all(&app_dir).ok();

      let db_path = app_dir.join("klyro.db");
      let conn = rusqlite::Connection::open(db_path).expect("db open");
      db::init(&conn).expect("db init");
      let db = Arc::new(Mutex::new(conn));

      // start HTTP server for streams and static HLS
      let (port_tx, port_rx) = std::sync::mpsc::channel();
      let db_clone = db.clone();
      std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
          let port = server::start_http(db_clone).await.expect("http");
          port_tx.send(port).ok();
        });
      });
      let http_port = port_rx.recv().unwrap_or(3007);

      // scheduler: every 5 minutes
      let db_for_tasks = db.clone();
      tauri::async_runtime::spawn(async move {
        tasks::start_scheduler(db_for_tasks).await;
      });

      app.manage(AppState { db, http_port });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      add_library_path, list_library_paths, scan_library,
      get_continue_watching, update_progress, get_movies, get_series,
      start_stream, get_http_base
    ])
    .run(tauri::generate_context!())
    .expect("error running Klyro");
}