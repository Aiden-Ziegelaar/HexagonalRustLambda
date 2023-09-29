pub mod models;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn default_time() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs().to_string()
}