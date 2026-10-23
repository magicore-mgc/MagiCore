//! Game install summary type.
//! Kiểu summary cài đặt Game.

use crate::engine::GameEngine;

/// Summary retained for source compatibility; unsupported installs never
/// fabricate one.
/// Giữ kiểu summary để tương thích source; install unsupported không bịa summary.
#[derive(Debug, Clone)]
pub struct InstallSummary {
    pub engine: GameEngine,
    pub installed_packages: Vec<String>,
    pub bytes_downloaded: u64,
    pub duration_ms: u64,
    pub verified: bool,
}
