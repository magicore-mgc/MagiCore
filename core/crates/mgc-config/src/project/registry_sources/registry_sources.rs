//! Serialized registry/index source selection configuration.
//! Cấu hình chọn registry/index source được tuần tự hóa.

use serde::{Deserialize, Serialize};

/// Registry/index source — `mgc.toml [[sources]]` multi-index policy.
/// Cấu hình registry/index source trong `mgc.toml [[sources]]`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceConfig {
    /// Stable id referenced by lockfile source_id — ID ổn định được lockfile tham chiếu.
    pub id: String,
    /// Base URL — URL gốc.
    pub url: String,
    /// Ecosystem served — ecosystem được phục vụ.
    #[serde(default)]
    pub ecosystem: String,
    /// Lower number wins among equal-specificity claims — số nhỏ hơn thắng khi độ cụ thể bằng nhau.
    #[serde(default)]
    pub priority: u64,
    /// Name patterns claimed — pattern tên package source nhận phục vụ.
    #[serde(default)]
    pub claims: Vec<String>,
    /// Private/internal source — source nội bộ.
    #[serde(default)]
    pub trusted: bool,
    /// Exact hosts allowed when trusted — host chính xác được phép khi trusted.
    #[serde(default)]
    pub allow_hosts: Vec<String>,
    /// Private CIDRs allowed when trusted — CIDR riêng được phép khi trusted.
    #[serde(default)]
    pub allow_cidrs: Vec<String>,
    /// URL schemes allowed when trusted — scheme URL được phép khi trusted.
    #[serde(default)]
    pub allow_protocols: Vec<String>,
}
