use rusqlite::{params, Connection, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn init(conn: &Connection) -> Result<()> {
  conn.pragma_update(None, "journal_mode", &"WAL")?;
  conn.execute_batch(r#"
    CREATE TABLE IF NOT EXISTS library_paths (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      path TEXT UNIQUE NOT NULL
    );
    CREATE TABLE IF NOT EXISTS media_items (
      id TEXT PRIMARY KEY,
      media_type TEXT NOT NULL, -- movie | series | season | episode
      title TEXT NOT NULL,
      year INTEGER,
      overview TEXT,
      poster_url TEXT,
      backdrop_url TEXT,
      runtime_sec INTEGER,
      tmdb_id INTEGER,
      imdb_id TEXT,
      created_at TEXT DEFAULT CURRENT_TIMESTAMP,
      updated_at TEXT DEFAULT CURRENT_TIMESTAMP
    );
    CREATE TABLE IF NOT EXISTS files (
      id TEXT PRIMARY KEY,
      media_id TEXT NOT NULL REFERENCES media_items(id) ON DELETE CASCADE,
      path TEXT UNIQUE NOT NULL,
      size INTEGER,
      duration_sec INTEGER,
      video_codec TEXT, audio_codec TEXT,
      width INTEGER, height INTEGER,
      last_seen_at TEXT DEFAULT CURRENT_TIMESTAMP
    );
    CREATE TABLE IF NOT EXISTS watch_progress (
      user_id TEXT NOT NULL,
      media_id TEXT NOT NULL,
      position_sec INTEGER NOT NULL,
      duration_sec INTEGER NOT NULL,
      updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
      PRIMARY KEY (user_id, media_id)
    );
    CREATE TABLE IF NOT EXISTS watchlist (
      user_id TEXT NOT NULL,
      media_id TEXT NOT NULL,
      added_at TEXT DEFAULT CURRENT_TIMESTAMP,
      PRIMARY KEY (user_id, media_id)
    );
    CREATE TABLE IF NOT EXISTS addons (
      id TEXT PRIMARY KEY,
      name TEXT,
      base_url TEXT,
      manifest_json TEXT,
      enabled INTEGER DEFAULT 1
    );
    CREATE TABLE IF NOT EXISTS iptv_channels (
      id TEXT PRIMARY KEY,
      name TEXT, logo TEXT, grp TEXT, url TEXT, number INTEGER
    );
  "#)?;
  Ok(())
}

pub fn add_library_path(db: &Arc<Mutex<Connection>>, path: &str) -> Result<()> {
  let conn = db.blocking_lock();
  conn.execute("INSERT OR IGNORE INTO library_paths(path) VALUES (?)", params![path])?;
  Ok(())
}

pub fn list_library_paths(db: &Arc<Mutex<Connection>>) -> Result<Vec<String>> {
  let conn = db.blocking_lock();
  let mut st = conn.prepare("SELECT path FROM library_paths ORDER BY id")?;
  let rows = st.query_map([], |r| Ok(r.get::<_, String>(0)?))?;
  Ok(rows.filter_map(|r| r.ok()).collect())
}

pub fn upsert_local_media_and_file(
  db: &Arc<Mutex<Connection>>,
  media_id: &str,
  title: &str,
  media_type: &str,
  file_id: &str,
  file_path: &str,
  duration_sec: Option<i64>,
  width: Option<i64>,
  height: Option<i64>,
  vcodec: Option<&str>,
  acodec: Option<&str>,
  size: Option<i64>,
) -> Result<()> {
  let conn = db.blocking_lock();
  conn.execute(
    "INSERT INTO media_items(id, media_type, title) VALUES(?,?,?)
     ON CONFLICT(id) DO UPDATE SET updated_at=CURRENT_TIMESTAMP",
    params![media_id, media_type, title],
  )?;
  conn.execute(
    "INSERT INTO files(id, media_id, path, size, duration_sec, video_codec, audio_codec, width, height)
     VALUES(?,?,?,?,?,?,?,?,?)
     ON CONFLICT(id) DO UPDATE SET last_seen_at=CURRENT_TIMESTAMP",
    params![file_id, media_id, file_path, size, duration_sec, vcodec, acodec, width, height],
  )?;
  Ok(())
}

pub fn list_media_by_type(db: &Arc<Mutex<Connection>>, ty: &str) -> rusqlite::Result<Vec<crate::models::MediaItem>> {
  let conn = db.blocking_lock();
  let mut st = conn.prepare("SELECT id, media_type, title, year, poster_url, runtime_sec FROM media_items WHERE media_type=? ORDER BY updated_at DESC")?;
  let rows = st.query_map([ty], |r| {
    Ok(crate::models::MediaItem {
      id: r.get(0)?, media_type: r.get(1)?, title: r.get(2)?,
      year: r.get::<_, Option<i64>>(3)?.map(|v| v as i32),
      poster_url: r.get::<_, Option<String>>(4)?,
      runtime_sec: r.get::<_, Option<i64>>(5)?.map(|v| v as i32),
    })
  })?;
  Ok(rows.filter_map(|r| r.ok()).collect())
}

pub fn update_progress(db: &Arc<Mutex<Connection>>, user_id: &str, media_id: &str, pos: i64, dur: i64) -> rusqlite::Result<()> {
  let conn = db.blocking_lock();
  conn.execute(
    "INSERT INTO watch_progress(user_id, media_id, position_sec, duration_sec) VALUES(?,?,?,?)
     ON CONFLICT(user_id, media_id) DO UPDATE SET position_sec=excluded.position_sec, duration_sec=excluded.duration_sec, updated_at=CURRENT_TIMESTAMP",
    params![user_id, media_id, pos, dur]
  )?;
  Ok(())
}

pub fn get_continue_watching(db: &Arc<Mutex<Connection>>) -> rusqlite::Result<Vec<crate::models::ContinueItem>> {
  let conn = db.blocking_lock();
  let mut st = conn.prepare(r#"
    SELECT m.id, m.title, m.poster_url, wp.position_sec, wp.duration_sec
    FROM watch_progress wp
    JOIN media_items m ON m.id = wp.media_id
    ORDER BY wp.updated_at DESC LIMIT 20
  "#)?;
  let rows = st.query_map([], |r| {
    Ok(crate::models::ContinueItem {
      id: r.get(0)?, title: r.get(1)?, poster_url: r.get(2)?,
      position_sec: r.get::<_, i64>(3)? as i32,
      duration_sec: r.get::<_, i64>(4)? as i32,
    })
  })?;
  Ok(rows.filter_map(|r| r.ok()).collect())
}