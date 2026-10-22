//! Represent typed dependency edges and origins.
//! Biểu diễn cạnh phụ thuộc và nguồn gốc có kiểu.

use super::super::package_identity::PackageKey;
use serde::{Deserialize, Serialize};

/// Structured dependency edge — Cạnh phụ thuộc có cấu trúc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    /// Full key of the target entry — Khóa đầy đủ của entry đích.
    pub target_key: PackageKey,
    /// Originally declared range — Range gốc được khai báo.
    pub range: String,
    /// Edge kind — Loại cạnh.
    pub kind: EdgeKind,
    /// Where the edge came from — Cạnh từ đâu ra.
    pub origin: EdgeOrigin,
    /// Evaluated marker (PEP 508 / cfg expression) — Marker đã evaluate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// Dependency edge kind — Loại cạnh phụ thuộc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeKind {
    Normal,
    Dev,
    Optional,
    Peer,
}

/// Edge origin — Nguồn gốc cạnh.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum EdgeOrigin {
    /// Declared in the project manifest — Khai báo trong manifest project.
    Manifest,
    /// Transitive edge from another locked entry — Cạnh bắc cầu từ entry
    /// đã lock khác.
    Transitive {
        /// Key of the entry carrying this edge — Khóa entry mang cạnh này.
        from_key: Box<PackageKey>,
    },
}

#[cfg(test)]
#[path = "test/dependency_edge.rs"]
mod tests;
