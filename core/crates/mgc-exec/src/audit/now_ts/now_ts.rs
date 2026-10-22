use std::time::{SystemTime, UNIX_EPOCH};

/// Timestamp hiện tại (giây kể từ epoch).
pub fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
