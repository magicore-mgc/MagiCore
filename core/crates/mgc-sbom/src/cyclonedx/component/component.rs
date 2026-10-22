//! CycloneDX component DTO — DTO component CycloneDX

use super::super::{ComponentType, Hash, License};
use serde::{Deserialize, Serialize};

/// Component — Component
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    /// Type (library, application, framework, etc.) — Loại
    #[serde(rename = "type")]
    pub component_type: ComponentType,
    /// BOM-Ref (unique ID) — BOM-Ref (ID duy nhất)
    #[serde(rename = "bom-ref")]
    pub bom_ref: String,
    /// Name — Tên
    pub name: String,
    /// Version — Phiên bản
    pub version: String,
    /// Package URL (purl) — URL package (purl)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
    /// Hashes — Hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<Hash>>,
    /// Licenses — Giấy phép
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<License>>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
