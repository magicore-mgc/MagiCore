//! Typed local errors for Python project manifest parsing.
//! Lỗi cục bộ có kiểu cho quá trình đọc manifest project Python.

use std::{error::Error, fmt};

use mgc_types::MgError;

#[derive(Debug)]
pub(super) enum PythonManifestParseError {
    Toml(toml::de::Error),
    UnsupportedDependencyGroups,
    InvalidDevelopmentRequirement,
}

impl fmt::Display for PythonManifestParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(error) => write!(formatter, "parse pyproject.toml: {error}"),
            Self::UnsupportedDependencyGroups => formatter.write_str(concat!(
                "pyproject.toml dependency groups are outside the MGC native Python lane; ",
                "only simple requirements in [dependency-groups].dev are supported"
            )),
            Self::InvalidDevelopmentRequirement => formatter
                .write_str("[dependency-groups].dev must contain only simple PEP 508 strings"),
        }
    }
}

impl Error for PythonManifestParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Toml(error) => Some(error),
            Self::UnsupportedDependencyGroups | Self::InvalidDevelopmentRequirement => None,
        }
    }
}

impl From<PythonManifestParseError> for MgError {
    fn from(error: PythonManifestParseError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
