//! Typed local errors for safe Python project manifest writing.
//! Lỗi cục bộ có kiểu cho quá trình ghi manifest Python an toàn.

use std::{error::Error, fmt};

use mgc_types::MgError;

#[derive(Debug)]
pub(super) enum PythonManifestWriteError {
    Toml(toml::de::Error),
    UnsupportedDependencyGroups,
    MissingProjectTable,
    InvalidRootTable,
    InvalidDependencyGroupsTable,
    Serialization(toml::ser::Error),
}

impl fmt::Display for PythonManifestWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(error) => write!(formatter, "parse pyproject.toml: {error}"),
            Self::UnsupportedDependencyGroups => formatter.write_str(concat!(
                "refusing to rewrite unsupported Python dependency groups; only ",
                "[dependency-groups].dev with simple requirements is MGC-managed"
            )),
            Self::MissingProjectTable => formatter.write_str("pyproject.toml missing [project]"),
            Self::InvalidRootTable => {
                formatter.write_str("pyproject.toml root must be a TOML table")
            }
            Self::InvalidDependencyGroupsTable => {
                formatter.write_str("dependency-groups must be a TOML table")
            }
            Self::Serialization(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl Error for PythonManifestWriteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Toml(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::UnsupportedDependencyGroups
            | Self::MissingProjectTable
            | Self::InvalidRootTable
            | Self::InvalidDependencyGroupsTable => None,
        }
    }
}

impl From<PythonManifestWriteError> for MgError {
    fn from(error: PythonManifestWriteError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
