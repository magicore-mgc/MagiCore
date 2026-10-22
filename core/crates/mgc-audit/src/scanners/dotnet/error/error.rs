//! Preserve NuGet lock parse context and serde source details.
//! Giữ context parse lock NuGet và cause serde gốc.

use mgc_types::MgError;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(in crate::scanners::dotnet) struct DotnetLockParseError(serde_json::Error);

impl Display for DotnetLockParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid packages.lock.json: {}", self.0)
    }
}

impl Error for DotnetLockParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl From<serde_json::Error> for DotnetLockParseError {
    fn from(error: serde_json::Error) -> Self {
        Self(error)
    }
}

impl From<DotnetLockParseError> for MgError {
    fn from(error: DotnetLockParseError) -> Self {
        Self::Anyhow(error.into())
    }
}
