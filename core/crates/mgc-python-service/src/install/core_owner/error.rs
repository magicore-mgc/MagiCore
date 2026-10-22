//! Local error categories for Python core identity and lock selection.
//! Nhóm lỗi cục bộ cho identity core Python và chọn package trong lock.

use std::{error::Error, fmt};

use mgc_types::MgError;

#[derive(Debug)]
pub(in crate::install) enum PythonCoreOwnerError {
    ReadMarker(anyhow::Error),
    ReadConfig(anyhow::Error),
    AmbiguousLegacyOwnership,
}

impl fmt::Display for PythonCoreOwnerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadMarker(error) => {
                write!(formatter, "cannot read Python project core marker: {error}")
            }
            Self::ReadConfig(error) => {
                write!(formatter, "cannot read Python project core config: {error}")
            }
            Self::AmbiguousLegacyOwnership => formatter.write_str(concat!(
                "Python entries in mgc.lock have ambiguous core ownership; run ",
                "`mgc install` to migrate the lock before launching Python"
            )),
        }
    }
}

impl Error for PythonCoreOwnerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadMarker(error) | Self::ReadConfig(error) => Some(error.as_ref()),
            Self::AmbiguousLegacyOwnership => None,
        }
    }
}

impl From<PythonCoreOwnerError> for MgError {
    fn from(error: PythonCoreOwnerError) -> Self {
        Self::Other(error.to_string())
    }
}
