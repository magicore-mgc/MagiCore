//! Public SBOM format selection — Lựa chọn định dạng SBOM public

/// SBOM format — Định dạng SBOM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbomFormat {
    /// CycloneDX JSON format — Định dạng CycloneDX JSON
    CycloneDx,
    /// SPDX 2.3 JSON format — Định dạng SPDX 2.3 JSON
    Spdx,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
