use anyhow::Result;
use std::fs;

pub fn parse_m3u(text: &str) -> Vec<(String, String)> {
  // returns Vec<(name,url)>, minimal
  let mut name = String::new(); let mut out = vec![];
  for line in text.lines() {
    if line.starts_with("#EXTINF:") {
      if let Some(pos) = line.rfind(",") {
        name = line[pos+1..].trim().to_string();
      }
    } else if line.starts_with("http") || line.starts_with("rtmp") {
      out.push((name.clone(), line.to_string()));
    }
  }
  out
}