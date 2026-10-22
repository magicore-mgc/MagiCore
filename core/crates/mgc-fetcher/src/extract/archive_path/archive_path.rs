//! Normalize archive entry paths and reject traversal.
//! Chuẩn hóa path entry archive và từ chối traversal.

use super::super::{ExtractionError, ExtractionResult};
use std::path::{Component, Path, PathBuf};

pub(in crate::extract) fn sanitize_archive_path(path: &Path) -> ExtractionResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(ExtractionError::EmptyArchivePath);
    }
    Ok(clean)
}
