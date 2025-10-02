use axum::{routing::get, Router, response::{IntoResponse}, extract::Path};
use std::{net::SocketAddr, sync::Arc, path::PathBuf};
use tokio::sync::Mutex;
use hyper::StatusCode;

pub async fn start_http(_db: Arc<Mutex<rusqlite::Connection>>) -> anyhow::Result<u16> {
  // dynamic port
  let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
  let port = listener.local_addr()?.port();
  let app = Router::new()
    .route("/", get(|| async { "Klyro Core" }))
    .route("/hls/:media_id/:file", get(hls_file))
    .route("/file/:media_id", get(stream_file));
  let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
  tokio::spawn(async move {
    let _ = server.await;
  });
  Ok(port)
}

async fn hls_file(Path((media_id, file)): Path<(String, String)>) -> impl IntoResponse {
  let base = tauri::api::path::app_data_dir(&tauri::Config::default()).unwrap().join("hls").join(&media_id);
  let fpath = base.join(&file);
  if fpath.exists() {
    if let Ok(bytes) = tokio::fs::read(&fpath).await {
      let ct = if file.ends_with(".m3u8") { "application/vnd.apple.mpegurl" } else { "video/MP2T" };
      return ([(hyper::header::CONTENT_TYPE, ct)], bytes).into_response();
    }
  }
  (StatusCode::NOT_FOUND, "not found").into_response()
}

async fn stream_file(Path(_media_id): Path<String>) -> impl IntoResponse {
  // optional: serve original file with range support later
  (StatusCode::NOT_IMPLEMENTED, "range streaming todo").into_response()
}