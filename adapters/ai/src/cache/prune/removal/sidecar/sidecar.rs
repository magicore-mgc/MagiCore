//! Find and remove uniquely-owned metadata sidecars — tìm và xóa sidecar có owner duy nhất.

use crate::cache::metadata::metadata_path;
use crate::cache::prune::error::{PruneError, PruneOutcome};
use std::fs;
use std::path::{Path, PathBuf};

pub(in crate::cache::prune) fn find_unique_sidecar(path: &Path) -> PruneOutcome<Option<PathBuf>> {
    let sidecar = metadata_path(path);
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let entries = fs::read_dir(parent)
        .map_err(|source| PruneError::io("inspect model cache siblings", source))?;

    for entry in entries {
        let entry = entry.map_err(|source| PruneError::io("read model cache sibling", source))?;
        let sibling = entry.path();
        if sibling == path || is_metadata_sidecar(&sibling) {
            continue;
        }

        let file_type = entry
            .file_type()
            .map_err(|source| PruneError::io("inspect model cache sibling", source))?;
        if (file_type.is_file() || file_type.is_dir()) && metadata_path(&sibling) == sidecar {
            return Ok(None);
        }
    }

    Ok(Some(sidecar))
}

fn is_metadata_sidecar(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.to_ascii_lowercase().ends_with(".meta.json"))
}
