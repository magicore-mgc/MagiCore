//! Classify local parser failures and preserve legacy rendered messages.
//! Phân loại lỗi parser cục bộ và giữ nguyên message legacy.

use mgc_types::MgError;
use std::fmt;

#[derive(Debug)]
pub(in crate::scanners::cargo_audit::parse) enum CargoAuditParseError {
    InvalidJson(serde_json::Error),
    UnrepresentablePackageName { name: String },
    UnparseablePackageVersion { name: String, version: String },
    CountMismatch { count: usize, actual: usize },
    FoundListMismatch,
}

impl CargoAuditParseError {
    pub(in crate::scanners::cargo_audit::parse) fn into_mg_error(self) -> MgError {
        MgError::Other(self.to_string())
    }
}

impl fmt::Display for CargoAuditParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(source) => write!(formatter, "invalid cargo-audit JSON: {source}"),
            Self::UnrepresentablePackageName { name } => write!(
                formatter,
                "cargo-audit finding for '{name}' has a package name mgc cannot represent"
            ),
            Self::UnparseablePackageVersion { name, version } => write!(
                formatter,
                "cargo-audit finding for '{name}' has an unparseable version '{version}'"
            ),
            Self::CountMismatch { count, actual } => write!(
                formatter,
                "cargo-audit JSON inconsistent: count says {count} but list holds {actual}"
            ),
            Self::FoundListMismatch => write!(
                formatter,
                "cargo-audit JSON inconsistent: found flag disagrees with list contents"
            ),
        }
    }
}

impl std::error::Error for CargoAuditParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidJson(source) => Some(source),
            _ => None,
        }
    }
}
