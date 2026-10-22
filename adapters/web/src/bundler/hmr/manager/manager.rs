//! HMR broadcast and version manager.
//! Broadcast và version manager của HMR.

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use tokio::sync::broadcast;

use super::super::{event::HmrEvent, watcher::watch_directory};

/// Shared event broadcaster used by the Web development server.
/// Bộ phát event dùng chung bởi Web development server.
pub struct HmrManager {
    tx: broadcast::Sender<HmrEvent>,
    version: Arc<AtomicU64>,
}

impl Default for HmrManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HmrManager {
    /// Create the existing bounded event channel and zero version.
    /// Tạo event channel có giới hạn và version ban đầu bằng không.
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self {
            tx,
            version: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Return the current update version using acquire ordering.
    /// Trả version update hiện tại với acquire ordering.
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Acquire)
    }

    /// Subscribe to events from this server instance.
    /// Đăng ký nhận event từ instance server này.
    pub fn subscribe(&self) -> broadcast::Receiver<HmrEvent> {
        self.tx.subscribe()
    }

    /// Watch a project directory and broadcast relevant file changes.
    /// Theo dõi thư mục project và phát thay đổi file liên quan.
    pub fn watch_dir(&self, path: &std::path::Path) -> anyhow::Result<notify::RecommendedWatcher> {
        watch_directory(self.tx.clone(), self.version.clone(), path).map_err(anyhow::Error::from)
    }
}

#[cfg(test)]
#[path = "test/manager.rs"]
mod tests;
