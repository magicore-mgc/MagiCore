//! Local errors for external Python manifest discovery.
//! Lỗi cục bộ khi dò manifest Python bên ngoài.

use std::{error::Error, fmt};

use mgc_types::MgError;

#[derive(Debug)]
pub(in crate::install) enum ExternalPythonManifestError {
    ReadDirectory(std::io::Error),
    ReadEntry(std::io::Error),
}

impl fmt::Display for ExternalPythonManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadDirectory(error) => {
                write!(
                    formatter,
                    "cannot inspect Python dependency manifests: {error}"
                )
            }
            Self::ReadEntry(error) => {
                write!(
                    formatter,
                    "cannot inspect Python dependency manifest entry: {error}"
                )
            }
        }
    }
}

impl Error for ExternalPythonManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadDirectory(error) | Self::ReadEntry(error) => Some(error),
        }
    }
}

impl From<ExternalPythonManifestError> for MgError {
    fn from(error: ExternalPythonManifestError) -> Self {
        Self::Other(error.to_string())
    }
}
