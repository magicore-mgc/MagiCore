//! Keeps operating-system errors attached to their storage operation.
//! Giữ lỗi hệ điều hành gắn với thao tác store phát sinh lỗi.

use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::doctor) enum StorageError {
    #[error("could not inspect store path '{path}': {source}")]
    InspectPath {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not create a private write probe in '{directory}': {source}")]
    CreateWriteProbe {
        directory: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not remove temporary write probe '{path}': {source}")]
    RemoveWriteProbe {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not query free space using existing directory '{path}': {source}")]
    AvailableSpace {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("no existing directory can be found for the store volume containing '{path}'")]
    NoVolumeAncestor { path: PathBuf },
    #[error("could not create missing store directory '{path}': {source}")]
    CreateStore {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
