// Hybrid HTTP client — online requests with cache-only fallback.
// HTTP client kết hợp — request online và fallback chỉ đọc cache.

use crate::offline::OfflineClient;
use anyhow::Result;
use reqwest::Client;
use std::time::Duration;

/// Hybrid client preserving the existing online/fallback policy.
/// Client kết hợp giữ nguyên chính sách online/fallback hiện tại.
pub struct HybridClient {
    online: Client,
    offline: OfflineClient,
    offline_mode: bool,
}

impl HybridClient {
    pub fn new(offline_ttl: Duration) -> Result<Self> {
        Ok(Self {
            online: Client::new(),
            offline: OfflineClient::new(offline_ttl),
            offline_mode: false,
        })
    }

    pub fn set_offline(&mut self, offline: bool) {
        self.offline_mode = offline;
    }

    pub async fn get(&self, url: &str) -> Result<Vec<u8>> {
        if self.offline_mode {
            return self.offline.get(url).await;
        }

        // Try the network first — ưu tiên thử mạng trước.
        match self.online.get(url).send().await {
            Ok(resp) if resp.status().is_success() => {
                let data = resp.bytes().await?.to_vec();
                // The current client does not persist successful responses.
                // Client hiện tại chưa lưu response thành công vào cache.
                // Return the owned buffer directly and avoid a second full-body allocation.
                // Trả trực tiếp buffer đang sở hữu để tránh cấp phát/copy toàn bộ body lần hai.
                Ok(data)
            }
            Ok(_) => {
                // Use the existing offline cache as fallback.
                // Dùng cache offline hiện có làm phương án dự phòng.
                self.offline.get(url).await
            }
            Err(_) => {
                // Use the existing offline cache after a network error.
                // Dùng cache offline hiện có sau lỗi mạng.
                self.offline.get(url).await
            }
        }
    }
}

#[cfg(test)]
#[path = "test/hybrid_client.rs"]
mod tests;
