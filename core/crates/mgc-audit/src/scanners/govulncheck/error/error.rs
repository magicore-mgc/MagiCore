//! Typed parse failures mapped to the legacy MgError::Other strings.
//! (Lỗi parse typed được ánh xạ về chuỗi MgError::Other hiện hữu.)

use mgc_types::MgError;
use std::fmt::{Display, Formatter};

/// Parse failure categories kept local to the Go scanner.
/// Nhóm lỗi parse được giữ cục bộ trong scanner Go.
#[derive(Debug)]
pub(in crate::scanners::govulncheck) enum GovulncheckError {
    InvalidJson(String),
    FindingWithoutOsvId,
    UnknownOsvId(String),
    InvalidPackageName(String),
}

impl Display for GovulncheckError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidJson(message) => write!(formatter, "invalid govulncheck JSON: {message}"),
            Self::FindingWithoutOsvId => formatter
                .write_str("govulncheck finding without an OSV id — stream contract violation"),
            Self::UnknownOsvId(osv_id) => write!(
                formatter,
                "govulncheck finding references OSV '{osv_id}' that never appeared in the stream",
            ),
            Self::InvalidPackageName(module_path) => write!(
                formatter,
                "govulncheck module path '{module_path}' cannot be represented as a package name",
            ),
        }
    }
}

impl std::error::Error for GovulncheckError {}

impl From<GovulncheckError> for MgError {
    fn from(error: GovulncheckError) -> Self {
        MgError::Other(error.to_string())
    }
}
