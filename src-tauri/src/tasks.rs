use std::sync::Arc;
use tokio::{sync::Mutex, time::{interval, Duration}};
use crate::scanner;

pub async fn start_scheduler(db: Arc<Mutex<rusqlite::Connection>>) {
  let mut tick = interval(Duration::from_secs(300)); // 5 minutes
  loop {
    tick.tick().await;
    let _ = scanner::scan_all(&db).await;
  }
}