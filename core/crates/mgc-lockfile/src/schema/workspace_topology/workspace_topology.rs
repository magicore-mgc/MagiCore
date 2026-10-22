//! Defines workspace members and cross-core dependency edges.
//! Định nghĩa thành viên workspace và cạnh dependency liên core.

use serde::{Deserialize, Serialize};

/// Workspace topology across cores — Topology workspace liên core.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceTopology {
    /// Member cores (e.g. ["web", "ai", "app", "lib"]) — Các core thành viên.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
    /// Cross-core dependency edges — Cạnh phụ thuộc liên core.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_core_edges: Vec<CrossEdge>,
}

/// One cross-core dependency edge — Một cạnh phụ thuộc liên core.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrossEdge {
    /// Consuming core — Core tiêu thụ.
    pub from_core: String,
    /// Providing core — Core cung cấp.
    pub to_core: String,
    /// Package name crossing the boundary — Tên package vượt ranh giới.
    pub package: String,
}
