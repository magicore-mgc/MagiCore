//! Define the patch record persisted in a lockfile.
//! Định nghĩa record patch được lưu trong lockfile.

use serde::{Deserialize, Serialize};

/// Record the patched package, post-patch hash and application timestamp.
/// Ghi package được patch, hash sau patch và thời điểm áp dụng.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPatch {
    pub name: String,
    pub version: String,
    pub sha256: String,
    /// ISO 8601 application timestamp.
    /// Thời điểm áp dụng theo ISO 8601.
    pub applied_at: String,
}

impl LockPatch {
    /// Construct a lockfile record without changing its supplied values.
    /// Tạo record lockfile và giữ nguyên các giá trị đầu vào.
    pub fn new(name: String, version: String, sha256: String, applied_at: String) -> Self {
        Self {
            name,
            version,
            sha256,
            applied_at,
        }
    }
}
