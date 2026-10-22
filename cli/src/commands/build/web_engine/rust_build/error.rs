//! Typed failures owned by managed Rust-source configuration.
//! Lỗi typed thuộc cấu hình source Rust do MGC quản lý.

use std::fmt;

#[derive(Debug)]
pub(super) enum RustBuildError {
    ManagedSourcePathNotUtf8,
}

impl fmt::Display for RustBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManagedSourcePathNotUtf8 => formatter
                .write_str("MagiCore Cargo source path must be valid UTF-8 on this platform"),
        }
    }
}

impl std::error::Error for RustBuildError {}
