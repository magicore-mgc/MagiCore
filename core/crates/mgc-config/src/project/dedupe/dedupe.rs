//! Serialized project dedupe preference.
//! Tùy chọn dedupe của project được tuần tự hóa.

use serde::{Deserialize, Serialize};

/// Dedupe config — opt-in via mgc.toml `[dedupe] prefer = true` (02 §2.1).
/// Bật ưu tiên gom dependency trùng lặp qua cấu hình project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DedupeConfig {
    /// Prefer dependency deduplication — ưu tiên dedupe dependency.
    #[serde(default)]
    pub prefer: bool,
}
