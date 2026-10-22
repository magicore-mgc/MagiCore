//! Remove measured cache entries — xóa entry đã chọn và sidecar an toàn.

use crate::cache::filesystem::logical_size::logical_size;
use crate::cache::prune::candidate_discovery::CacheCandidate;
use crate::cache::prune::error::{PruneError, PruneOutcome};
use crate::cache::prune::removal::find_unique_sidecar;
use std::fs;
use std::path::Path;

pub(in crate::cache::prune) fn remove_candidate(
    candidate: &CacheCandidate,
) -> PruneOutcome<Option<u64>> {
    let Some(content_bytes) = remove_entry(&candidate.path)? else {
        return Ok(None);
    };
    let sidecar_bytes = candidate
        .metadata_sidecar
        .as_deref()
        .map(remove_regular_sidecar)
        .transpose()?
        .unwrap_or_default();
    Ok(Some(content_bytes.saturating_add(sidecar_bytes)))
}

pub(in crate::cache::prune) fn remove_model_path(path: &Path) -> PruneOutcome<Option<u64>> {
    match fs::symlink_metadata(path) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(PruneError::io("inspect model cache entry", source)),
    }

    let sidecar = find_unique_sidecar(path)?;
    let Some(content_bytes) = remove_entry(path)? else {
        return Ok(None);
    };
    let sidecar_bytes = sidecar
        .as_deref()
        .map(remove_regular_sidecar)
        .transpose()?
        .unwrap_or_default();
    Ok(Some(content_bytes.saturating_add(sidecar_bytes)))
}

fn remove_entry(path: &Path) -> PruneOutcome<Option<u64>> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(PruneError::io("inspect model cache entry", source)),
    };

    let file_type = metadata.file_type();
    if file_type.is_symlink() {
        return Err(PruneError::SymbolicLinkEntry);
    }

    let bytes = if file_type.is_file() || file_type.is_dir() {
        logical_size(path).map_err(PruneError::from)?
    } else {
        return Err(PruneError::UnsupportedEntry);
    };

    if file_type.is_dir() {
        fs::remove_dir_all(path)
            .map_err(|source| PruneError::io("remove model cache directory", source))?;
    } else {
        fs::remove_file(path)
            .map_err(|source| PruneError::io("remove model cache file", source))?;
    }

    Ok(Some(bytes))
}

fn remove_regular_sidecar(path: &Path) -> PruneOutcome<u64> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(source) => return Err(PruneError::io("inspect cache metadata sidecar", source)),
    };

    if !metadata.file_type().is_file() {
        return Ok(0);
    }

    let bytes = metadata.len();
    fs::remove_file(path)
        .map_err(|source| PruneError::io("remove cache metadata sidecar", source))?;
    Ok(bytes)
}
