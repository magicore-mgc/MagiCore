//! Defines normalized dependency requests and registry feature options.
//! Định nghĩa yêu cầu dependency chuẩn hóa và tùy chọn feature registry.

use super::super::{PackageName, VersionRange};
use crate::error::MgResult;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DependencySpec {
    pub name: PackageName,
    pub range: VersionRange,
    pub dev: bool,
    pub optional: bool,
    pub peer: bool,
    /// Registry features selected for this dependency (Cargo only today).
    /// Feature registry được chọn cho dependency (hiện chỉ Cargo dùng).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
    /// Whether registry default features are enabled (Cargo only today).
    /// Có bật feature mặc định của registry hay không (hiện chỉ Cargo dùng).
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub default_features: bool,
}

impl DependencySpec {
    pub fn new(name: PackageName, range: VersionRange) -> Self {
        Self {
            name,
            range,
            dev: false,
            optional: false,
            peer: false,
            features: Vec::new(),
            default_features: true,
        }
    }

    pub fn parse(input: &str) -> MgResult<Self> {
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if let Some(idx) = input.rfind('@')
            && idx > 0
        {
            let name = PackageName::new(&input[..idx])?;
            let range = VersionRange::parse(&input[idx + 1..])?;
            return Ok(Self::new(name, range));
        }
        Ok(Self::new(PackageName::new(input)?, VersionRange::star()))
    }
}

fn default_true() -> bool {
    true
}

fn is_true(value: &bool) -> bool {
    *value
}
