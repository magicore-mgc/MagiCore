//! Typed failures for declared executable selection.
//! Lỗi có kiểu khi chọn executable đã khai báo.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[allow(clippy::enum_variant_names)] // The suffix keeps each bin-selection diagnostic explicit.
pub(crate) enum BinSelectionError {
    InvalidBin,
    UndeclaredBin,
    AmbiguousBin,
}

impl Display for BinSelectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBin => write!(
                formatter,
                "package runner: invalid executable name; use a simple declared bin name"
            ),
            Self::UndeclaredBin => write!(
                formatter,
                "package runner: selected executable is not declared by the requested package"
            ),
            Self::AmbiguousBin => write!(
                formatter,
                "package runner: package declares multiple executables; select one with --bin NAME"
            ),
        }
    }
}

impl std::error::Error for BinSelectionError {}

pub fn registry_package_invalid_bin() -> AnyhowError {
    AnyhowError::new(BinSelectionError::InvalidBin)
}

pub fn registry_package_undeclared_bin() -> AnyhowError {
    AnyhowError::new(BinSelectionError::UndeclaredBin)
}

pub fn registry_package_ambiguous_bin() -> AnyhowError {
    AnyhowError::new(BinSelectionError::AmbiguousBin)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
