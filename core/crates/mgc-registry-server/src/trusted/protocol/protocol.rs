//! Defines supported registry protocols and canonical wire strings.
//! Định nghĩa protocol registry được hỗ trợ cùng wire string chuẩn.

use super::super::prelude::*;

/// Registry protocol used to keep a package binding unique across ecosystems.
/// Giao thức registry giúp binding không bị trộn giữa các hệ sinh thái.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustedProtocol {
    Npm,
    Pypi,
    Oci,
}

impl TrustedProtocol {
    pub fn parse(value: &str) -> Result<Self, StatusCode> {
        match value {
            "npm" => Ok(Self::Npm),
            "pypi" => Ok(Self::Pypi),
            "oci" => Ok(Self::Oci),
            _ => Err(StatusCode::BAD_REQUEST),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pypi => "pypi",
            Self::Oci => "oci",
        }
    }
}
