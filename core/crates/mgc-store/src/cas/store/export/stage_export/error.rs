//! Owns typed staging failures until the export boundary maps them outward.
//! Giữ lỗi staging có kiểu tới ranh giới export rồi mới chuyển ra ngoài.

use std::io;
use std::path::{Path, PathBuf};

use crate::cas::store::StoreError;

#[derive(Debug, thiserror::Error)]
pub(in crate::cas::store::export) enum StageExportError {
    #[error("copy blob for export failed: {source}")]
    Copy {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl StageExportError {
    pub(super) fn copy(path: &Path, source: io::Error) -> Self {
        Self::Copy {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(in crate::cas::store::export) fn into_store_error(self) -> StoreError {
        match self {
            Self::Copy { path, source } => StoreError::Io {
                path,
                msg: format!("copy blob for export failed: {source}"),
            },
        }
    }
}
