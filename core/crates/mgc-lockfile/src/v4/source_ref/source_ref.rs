//! Represent registry source identity and transport claims.
//! Biểu diễn identity registry source và claim transport.

use crate::ecosystem_tag::EcosystemTag;
use serde::{Deserialize, Serialize};

/// A registry/index source — Một nguồn registry/index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    /// Stable id referenced by `PackageKey.source_id` — Id ổn định mà
    /// `PackageKey.source_id` trỏ tới.
    pub id: String,
    /// Canonical base URL (no trailing slash, lowercase scheme+host).
    pub url: String,
    /// Ecosystem this source serves — Hệ sinh thái nguồn này phục vụ.
    pub ecosystem: EcosystemTag,
    /// Lower number wins among equal-specificity claims — Số nhỏ thắng
    /// giữa các claim cùng độ đặc hiệu.
    #[serde(default)]
    pub priority: u64,
    /// Name patterns this source claims (`*`, `corp-*`, `@corp/*`,
    /// exact names) — Pattern tên nguồn này tuyên bố sở hữu.
    #[serde(default)]
    pub claims: Vec<String>,
    /// Private/internal source (never overridden by public ones).
    #[serde(default)]
    pub trusted: bool,
    /// Exact hosts allowed when `trusted` (transport guard §5.3.1).
    #[serde(default)]
    pub allow_hosts: Vec<String>,
    /// Private CIDRs allowed when `trusted` — Dải CIDR private cho phép.
    #[serde(default)]
    pub allow_cidrs: Vec<String>,
    /// URL schemes allowed when `trusted` (default HTTPS only).
    #[serde(default)]
    pub allow_protocols: Vec<String>,
}

#[cfg(test)]
#[path = "test/source_ref.rs"]
mod tests;
