//! CycloneDX metadata DTO — DTO metadata CycloneDX

use super::super::{Component, Tool};
use serde::{Deserialize, Serialize};

/// Metadata — Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Timestamp (ISO 8601) — Timestamp (ISO 8601)
    pub timestamp: String,
    /// Tools used to generate BOM — Công cụ tạo BOM
    pub tools: Vec<Tool>,
    /// Component being described — Component được mô tả
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<Component>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
