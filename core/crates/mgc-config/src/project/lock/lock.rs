//! Serialized lock policy, writer-lock timeouts and lockfile schema target.
//! Policy lock, timeout writer-lock và schema ghi lockfile được tuần tự hóa.

use serde::{Deserialize, Serialize};

/// Lock config — `mgc.toml [lock]` with optional policy and writer settings.
/// Cấu hình `[lock]` gồm policy và thiết lập writer tùy chọn.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockConfig {
    /// Signature policy: `off` | `warn` | `require` — policy chữ ký.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Writer-lock acquire timeout in milliseconds — timeout chờ writer-lock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acquire_timeout_ms: Option<u64>,
    /// Stale-temp grace period in seconds before cleanup may unlink — thời gian chờ dọn temp cũ.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tmp_grace_secs: Option<u64>,
    /// New lockfile write target schema — schema đích khi ghi lockfile mới.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}
