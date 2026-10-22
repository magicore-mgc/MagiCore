//! Typed failures from the dependency bundle operation.
//! Lỗi có kiểu từ thao tác bundle dependency.

use std::fmt;

/// Failure categories retained inside the dependency-bundling owner.
/// Nhóm lỗi được giữ bên trong owner dependency-bundling.
#[derive(Debug)]
pub(in crate::bundler::dependency_cache) enum BundleError {
    Esbuild(Vec<String>),
    MetafileInputs(super::super::build::MetafileInputsError),
}

impl fmt::Display for BundleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Esbuild(messages) => formatter.write_str(&messages.join("; ")),
            Self::MetafileInputs(error) => {
                write!(formatter, "dependency bundle input rejected: {error}")
            }
        }
    }
}

impl std::error::Error for BundleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Esbuild(_) => None,
            Self::MetafileInputs(error) => Some(error),
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
