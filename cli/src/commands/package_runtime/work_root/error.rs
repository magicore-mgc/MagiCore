//! Typed failures for unsafe MGC-managed cache paths.
//! Lỗi có kiểu khi đường dẫn cache do MGC quản lý không an toàn.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum WorkRootError {
    UnsafeCachePath { path: PathBuf },
}

impl Display for WorkRootError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsafeCachePath { path } => write!(
                formatter,
                "package runner: cache directory '{}' must be a real directory without symlinks",
                path.display()
            ),
        }
    }
}

impl std::error::Error for WorkRootError {}

pub fn registry_package_cache_path_unsafe(path: &Path) -> AnyhowError {
    AnyhowError::new(WorkRootError::UnsafeCachePath {
        path: path.to_path_buf(),
    })
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
