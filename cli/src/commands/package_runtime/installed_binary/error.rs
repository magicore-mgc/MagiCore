//! Typed failures for installed package and executable containment.
//! Lỗi có kiểu cho containment package và executable đã cài.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum InstalledBinaryError {
    NoBinary { bin: String, package: String },
    BinaryOutsideProject,
    BinTargetMismatch,
}

impl Display for InstalledBinaryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBinary { bin, package } => write!(
                formatter,
                "package runner: no binary '{bin}' found for package '{package}'. Is it an executable package?"
            ),
            Self::BinaryOutsideProject => write!(
                formatter,
                "package runner: executable resolves outside the verified temporary project"
            ),
            Self::BinTargetMismatch => write!(
                formatter,
                "package runner: executable does not match the requested package's declared target"
            ),
        }
    }
}

impl std::error::Error for InstalledBinaryError {}

pub fn registry_package_no_binary(bin: &str, package: &str) -> AnyhowError {
    AnyhowError::new(InstalledBinaryError::NoBinary {
        bin: bin.to_owned(),
        package: package.to_owned(),
    })
}

pub fn registry_package_binary_outside_project() -> AnyhowError {
    AnyhowError::new(InstalledBinaryError::BinaryOutsideProject)
}

pub fn registry_package_bin_target_mismatch() -> AnyhowError {
    AnyhowError::new(InstalledBinaryError::BinTargetMismatch)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
