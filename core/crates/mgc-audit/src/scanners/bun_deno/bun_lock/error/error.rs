//! Typed Bun lock parse failures — lỗi parse lock Bun có kiểu riêng.
//! Preserve the legacy messages and keep serde's parse error as a source.
//! Giữ nguyên thông điệp cũ và lưu lỗi parse serde làm source.

use mgc_types::MgError;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(in crate::scanners::bun_deno::bun_lock) enum BunLockError {
    InvalidJson(serde_json::Error),
    PackageEntryNotArray(String),
    FirstEntryNotString(String),
}

impl Display for BunLockError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(source) => write!(formatter, "invalid bun.lock: {source}"),
            Self::PackageEntryNotArray(key) => {
                write!(formatter, "bun.lock package '{key}' entry must be an array")
            }
            Self::FirstEntryNotString(key) => write!(
                formatter,
                "bun.lock package '{key}' first entry must be a name@version string"
            ),
        }
    }
}

impl Error for BunLockError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidJson(source) => Some(source),
            Self::PackageEntryNotArray(_) | Self::FirstEntryNotString(_) => None,
        }
    }
}

impl From<BunLockError> for MgError {
    fn from(error: BunLockError) -> Self {
        Self::Other(error.to_string())
    }
}
