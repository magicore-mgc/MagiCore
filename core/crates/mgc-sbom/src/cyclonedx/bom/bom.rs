//! CycloneDX BOM model and defaults — Model BOM CycloneDX và giá trị mặc định

use super::super::{Component, Dependency, Metadata, Tool};
use serde::{Deserialize, Serialize};

/// CycloneDX BOM (Bill of Materials) — BOM CycloneDX
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bom {
    /// BOM format (always "CycloneDX") — Định dạng BOM
    pub bom_format: String,
    /// Spec version (e.g., "1.5") — Phiên bản spec
    pub spec_version: String,
    /// Serial number (UUID) — Số serial (UUID)
    pub serial_number: String,
    /// Version (increment for updates) — Phiên bản (tăng khi cập nhật)
    pub version: u32,
    /// Metadata — Metadata
    pub metadata: Metadata,
    /// Components list — Danh sách components
    pub components: Vec<Component>,
    /// Dependencies graph — Đồ thị dependencies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,
}

impl Bom {
    /// Create new BOM — Tạo BOM mới
    pub fn new() -> Self {
        Self {
            bom_format: "CycloneDX".to_string(),
            spec_version: "1.5".to_string(),
            serial_number: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            version: 1,
            metadata: Metadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                tools: vec![Tool {
                    vendor: "MagiCore".to_string(),
                    name: "mgc".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                }],
                component: None,
            },
            components: Vec::new(),
            dependencies: None,
        }
    }
}

impl Default for Bom {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
