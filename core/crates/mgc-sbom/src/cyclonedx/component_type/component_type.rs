//! CycloneDX component type enum — Enum loại component CycloneDX

use serde::{Deserialize, Serialize};

/// Component type — Loại component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Application,
    Framework,
    Library,
    Container,
    Device,
    File,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
