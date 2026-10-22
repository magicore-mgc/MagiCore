//! Typed failures owned by native-engine artifact verification.
//! Lỗi typed thuộc khâu xác minh artifact native-engine.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum NativeEngineError {
    BinaryNotFound { path: PathBuf },
}

impl fmt::Display for NativeEngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinaryNotFound { path } => write!(
                formatter,
                "native engine build completed but binary '{}' was not found",
                path.display()
            ),
        }
    }
}

impl std::error::Error for NativeEngineError {}
