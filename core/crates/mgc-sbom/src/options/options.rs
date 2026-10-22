//! Public SBOM generation options and defaults — Tùy chọn tạo SBOM public và giá trị mặc định

use crate::SbomFormat;

/// SBOM generation options — Tùy chọn tạo SBOM
#[derive(Debug, Clone)]
pub struct SbomOptions {
    /// Include dev dependencies — Bao gồm dev dependencies
    pub include_dev: bool,
    /// Include licenses — Bao gồm giấy phép
    pub include_licenses: bool,
    /// Include hashes — Bao gồm hash
    pub include_hashes: bool,
    /// SBOM format — Định dạng SBOM
    pub format: SbomFormat,
}

impl Default for SbomOptions {
    fn default() -> Self {
        Self {
            include_dev: false,
            include_licenses: true,
            include_hashes: true,
            format: SbomFormat::CycloneDx,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
