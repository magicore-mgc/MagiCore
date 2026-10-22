//! CycloneDX tool DTO — DTO tool CycloneDX

use serde::{Deserialize, Serialize};

/// Tool — Công cụ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Vendor — Nhà cung cấp
    pub vendor: String,
    /// Name — Tên
    pub name: String,
    /// Version — Phiên bản
    pub version: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
