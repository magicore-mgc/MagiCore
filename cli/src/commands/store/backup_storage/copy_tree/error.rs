//! Errors at the tree-copy orchestration boundary.
//! Lỗi tại biên điều phối copy cây.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::store) enum CopyTreeError {
    #[error(
        "tree source and destination overlap: source={source_path}, destination={destination_path}"
    )]
    SourceDestinationOverlap {
        source_path: PathBuf,
        destination_path: PathBuf,
    },
    #[error("destination root changed during preparation: expected={expected}, actual={actual}")]
    DestinationRootChanged { expected: PathBuf, actual: PathBuf },
    #[error("prepare destination root: {0}")]
    Destination(#[source] anyhow::Error),
    #[error("copy source tree: {0}")]
    TreeCopy(#[source] anyhow::Error),
}
