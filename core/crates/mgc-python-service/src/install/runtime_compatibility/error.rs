//! Local errors for runtime path and wheel importability checks.
//! Lỗi cục bộ khi kiểm tra path runtime và khả năng import wheel.

use std::{error::Error, fmt};

use mgc_types::MgError;

#[derive(Debug)]
pub(in crate::install) enum PythonRuntimeCompatibilityError {
    Resolver(MgError),
    ArtifactNotImportable {
        name: String,
        version: String,
        filename: String,
        guidance: &'static str,
    },
}

impl fmt::Display for PythonRuntimeCompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolver(error) => write!(formatter, "{error}"),
            Self::ArtifactNotImportable {
                name,
                version,
                filename,
                guidance,
            } => write!(
                formatter,
                "native Python install cannot materialize '{name}'@{version} ({filename}); {guidance}; no external package manager was invoked"
            ),
        }
    }
}

impl Error for PythonRuntimeCompatibilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Resolver(error) => Some(error),
            Self::ArtifactNotImportable { .. } => None,
        }
    }
}

impl From<PythonRuntimeCompatibilityError> for MgError {
    fn from(error: PythonRuntimeCompatibilityError) -> Self {
        match error {
            PythonRuntimeCompatibilityError::Resolver(error) => error,
            error @ PythonRuntimeCompatibilityError::ArtifactNotImportable { .. } => {
                Self::Other(error.to_string())
            }
        }
    }
}
