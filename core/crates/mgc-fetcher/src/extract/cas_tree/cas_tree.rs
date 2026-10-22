//! Materialize validated archive directory structure safely.
//! Tạo cấu trúc thư mục archive đã xác minh một cách an toàn.

use super::super::cas_spool::StagedArchive;
use super::super::{ExtractionError, ExtractionResult};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(in crate::extract) fn prepare_destination(
    root: &Path,
    staged: &StagedArchive,
) -> ExtractionResult<()> {
    for directory in &staged.directories {
        let relative = directory
            .strip_prefix(root)
            .map_err(|_| ExtractionError::PathEscapes(directory.clone()))?;
        super::super::extract_target::ensure_extract_directory(root, relative)?;
    }
    let mut parents = HashSet::new();
    for file in &staged.files {
        let parent = file
            .path
            .parent()
            .ok_or(ExtractionError::DestinationHasNoParent)?;
        parents.insert(parent.to_path_buf());
    }
    let mut parents: Vec<PathBuf> = parents.into_iter().collect();
    parents.sort_by_key(|path| path.components().count());
    for parent in parents {
        let relative = parent
            .strip_prefix(root)
            .map_err(|_| ExtractionError::PathEscapes(parent.clone()))?;
        super::super::extract_target::ensure_extract_directory(root, relative)?;
    }
    for file in &staged.files {
        let relative = file
            .path
            .strip_prefix(root)
            .map_err(|_| ExtractionError::PathEscapes(file.path.clone()))?;
        super::super::extract_target::ensure_extract_file_target(root, relative)?;
    }
    Ok(())
}
