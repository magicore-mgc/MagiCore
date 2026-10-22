//! CycloneDX license DTO — DTO license CycloneDX

use serde::{Deserialize, Serialize};

/// License — Giấy phép
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// License ID (SPDX) — ID giấy phép (SPDX)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// License name — Tên giấy phép
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
