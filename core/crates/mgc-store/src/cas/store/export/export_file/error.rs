//! Owns source-open failures and their stable CAS export mapping.
//! Quản lý lỗi mở nguồn và mapping ổn định của CAS export.

use std::fs::File;
use std::io;
use std::path::Path;

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::StoreError;
use mgc_platform::{OpenRegularFileError, open_regular_file_no_follow};

#[derive(Debug, thiserror::Error)]
pub(super) enum ExportSourceError {
    #[error("content not found: {digest}")]
    NotFound { digest: String },
    #[error("CAS blob path is not a regular file")]
    NotRegular,
    #[cfg(not(any(unix, windows)))]
    #[error("secure regular-file opening is unavailable on this platform")]
    UnsupportedPlatform,
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub(super) fn open_cas_blob_source(
    path: &Path,
    hash: &IntegrityHash,
) -> Result<File, ExportSourceError> {
    if !path.exists() {
        return Err(ExportSourceError::NotFound {
            digest: hash.as_hex().to_string(),
        });
    }

    open_regular_file_no_follow(path).map_err(|error| match error {
        OpenRegularFileError::Symlink
        | OpenRegularFileError::NotRegular
        | OpenRegularFileError::OpenedNotRegular => ExportSourceError::NotRegular,
        #[cfg(windows)]
        OpenRegularFileError::ReparsePoint => ExportSourceError::NotRegular,
        #[cfg(not(any(unix, windows)))]
        OpenRegularFileError::UnsupportedPlatform => ExportSourceError::UnsupportedPlatform,
        OpenRegularFileError::Io(source) if source.kind() == io::ErrorKind::NotFound => {
            ExportSourceError::NotFound {
                digest: hash.as_hex().to_string(),
            }
        }
        OpenRegularFileError::Io(source) => ExportSourceError::Io(source),
    })
}

impl ExportSourceError {
    pub(super) fn into_store_error(self, path: &Path) -> StoreError {
        match self {
            Self::NotFound { digest } => StoreError::NotFound(digest),
            error => StoreError::Io {
                path: path.to_path_buf(),
                msg: error.to_string(),
            },
        }
    }
}

#[cfg(test)]
#[path = "error/test/mod.rs"]
mod test;
