//! Represent package-instance identity in the v4 graph.
//! Biểu diễn identity instance package trong đồ thị v4.

use crate::ecosystem_tag::EcosystemTag;
use serde::{Deserialize, Serialize};

/// Unique identity of ONE locked instance. Two entries sharing a
/// PackageKey are the same instance; anything else (peer set, features,
/// target, source) makes a different instance.
/// Danh tính duy nhất của MỘT instance đã lock. Hai entry cùng
/// PackageKey là một instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageKey {
    /// Ecosystem of the pin — Hệ sinh thái của pin.
    pub ecosystem: EcosystemTag,
    /// Canonical name (`canonical_name`) — Tên canonical.
    pub name: String,
    /// Canonical version string (`canonical_version`) — Chuỗi version canonical.
    pub version: String,
    /// Foreign key into the lockfile `[sources]` table (§5) — Khóa ngoại
    /// tới bảng `[sources]` của lockfile.
    pub source_id: String,
    /// What makes this instance distinct beyond name/version/source.
    #[serde(default)]
    pub variant: VariantKey,
}

/// Per-instance variation axis — Trục biến thể của từng instance.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VariantKey {
    /// Digest of the resolved peer set (`peer_context_digest`), None when
    /// the ecosystem has no peers — Digest của tập peer đã chốt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_context: Option<String>,
    /// Sorted enabled feature set (cargo features, extras) — Tập feature
    /// đã bật (đã sắp xếp).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<Vec<String>>,
    /// Canonical target tuple (`target_tuple`) or None for
    /// platform-independent packages — Tuple target canonical hoặc None
    /// cho package không phụ thuộc nền tảng.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[cfg(test)]
#[path = "test/package_identity.rs"]
mod tests;
