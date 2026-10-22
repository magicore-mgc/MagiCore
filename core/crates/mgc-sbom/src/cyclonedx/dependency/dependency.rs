//! CycloneDX dependency DTO — DTO dependency CycloneDX

use serde::{Deserialize, Serialize};

/// Dependency — Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    /// Reference to component — Tham chiếu đến component
    #[serde(rename = "ref")]
    pub dependency_ref: String,
    /// List of dependencies — Danh sách dependencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
