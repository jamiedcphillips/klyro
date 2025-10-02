use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
  pub id: String,
  pub media_type: String,
  pub title: String,
  pub year: Option<i32>,
  pub poster_url: Option<String>,
  pub runtime_sec: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueItem {
  pub id: String,
  pub title: String,
  pub poster_url: Option<String>,
  pub position_sec: i32,
  pub duration_sec: i32,
}