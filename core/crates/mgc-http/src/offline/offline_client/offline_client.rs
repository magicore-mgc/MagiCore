// Offline HTTP client — deterministic cache-only reads and guarded mutations.
// HTTP client ngoại tuyến — đọc cache xác định và chặn thao tác ghi.

use crate::offline::error::OfflineError;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Offline HTTP client — cache-only request behavior.
/// HTTP client ngoại tuyến — chỉ xử lý request từ cache.
pub struct OfflineClient {
    pub cache: HashMap<String, (Vec<u8>, SystemTime)>,
    ttl: Duration,
}

impl OfflineClient {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            ttl,
        }
    }

    pub fn with_cache(mut self, cache: HashMap<String, (Vec<u8>, SystemTime)>) -> Self {
        self.cache = cache;
        self
    }

    /// Read cached bytes while offline — đọc dữ liệu cache khi ngoại tuyến.
    pub async fn get(&self, url: &str) -> Result<Vec<u8>> {
        if let Some((data, ts)) = self.cache.get(url) {
            let age = SystemTime::now()
                .duration_since(*ts)
                .unwrap_or(Duration::MAX);

            if age > self.ttl {
                // Stale metadata stays usable, but its freshness limitation is reported.
                // Metadata cũ vẫn dùng được nhưng cảnh báo rõ giới hạn về độ mới.
                eprintln!(
                    "WARNING: stale metadata for {} (age: {:.0}s, ttl: {:.0}s) - cannot verify latest version from registry",
                    url,
                    age.as_secs(),
                    self.ttl.as_secs()
                );
                return Ok(data.clone());
            }
            return Ok(data.clone());
        }

        Err(anyhow::Error::new(OfflineError::CacheMiss {
            url: url.to_string(),
        }))
    }

    /// Reject mutations while offline — chặn thao tác thay đổi khi ngoại tuyến.
    pub async fn put(&self, _url: &str, _body: Vec<u8>) -> Result<Vec<u8>> {
        Err(anyhow::Error::new(OfflineError::MethodNotAllowed {
            method: "PUT",
        }))
    }

    pub async fn post(&self, _url: &str, _body: Vec<u8>) -> Result<Vec<u8>> {
        Err(anyhow::Error::new(OfflineError::MethodNotAllowed {
            method: "POST",
        }))
    }

    pub async fn delete(&self, _url: &str) -> Result<Vec<u8>> {
        Err(anyhow::Error::new(OfflineError::MethodNotAllowed {
            method: "DELETE",
        }))
    }

    /// Load the existing JSON cache format — nạp định dạng JSON cache hiện có.
    pub fn load_cache(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(OfflineError::ReadCache)
            .map_err(anyhow::Error::new)?;
        let cache: HashMap<String, (Vec<u8>, SystemTime)> = serde_json::from_str(&content)
            .map_err(OfflineError::ParseCache)
            .map_err(anyhow::Error::new)?;
        Ok(Self::new(Duration::from_secs(600)).with_cache(cache))
    }

    /// Save the existing JSON cache format — lưu định dạng JSON cache hiện có.
    pub fn save_cache(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string(&self.cache)
            .map_err(OfflineError::SerializeCache)
            .map_err(anyhow::Error::new)?;
        std::fs::write(path, json)
            .map_err(OfflineError::WriteCache)
            .map_err(anyhow::Error::new)?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/offline_client.rs"]
mod tests;
