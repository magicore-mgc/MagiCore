//! Recompute package content signatures without following symlinks.
//! Tính lại chữ ký nội dung package mà không đi theo symlink.

use std::io::Read;
use std::path::Path;

use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::cache::ExtractedPackageMarker;
use crate::install::package_marker::{
    SHA256_READ_BUFFER_BYTES, extracted_package_marker_path, metadata_is_executable,
    path_to_signature_string, update_tree_hasher,
};

pub fn extracted_content_matches(root: &Path, expected: &ExtractedPackageMarker) -> MgResult<bool> {
    if expected.file_tree_sha256.is_empty() {
        return Ok(false);
    }

    let mut files = Vec::<(String, u64, bool, String)>::new();
    for entry in WalkDir::new(root).min_depth(1) {
        let entry = entry.map_err(|err| {
            MgError::Other(format!(
                "failed to walk extracted package '{}': {err}",
                root.display()
            ))
        })?;
        if entry.path() == extracted_package_marker_path(root) {
            continue;
        }
        if entry.file_type().is_dir() {
            continue;
        }
        if !entry.file_type().is_file() {
            return Ok(false);
        }
        let rel = entry.path().strip_prefix(root).map_err(|err| {
            MgError::Other(format!(
                "failed to inspect extracted package path '{}': {}",
                entry.path().display(),
                err
            ))
        })?;
        let metadata = std::fs::symlink_metadata(entry.path()).map_err(|err| {
            MgError::Other(format!(
                "failed to inspect extracted package file '{}': {err}",
                entry.path().display()
            ))
        })?;
        if !metadata.file_type().is_file() {
            return Ok(false);
        }
        let mut file = std::fs::File::open(entry.path()).map_err(|err| {
            MgError::Other(format!(
                "failed to open extracted package file '{}': {err}",
                entry.path().display()
            ))
        })?;
        let mut content_hasher = Sha256::new();
        let mut actual_size = 0u64;
        let mut buffer = [0u8; SHA256_READ_BUFFER_BYTES];
        loop {
            let read = file.read(&mut buffer).map_err(|err| {
                MgError::Other(format!(
                    "failed to hash extracted package file '{}': {err}",
                    entry.path().display()
                ))
            })?;
            if read == 0 {
                break;
            }
            actual_size = actual_size.checked_add(read as u64).ok_or_else(|| {
                MgError::Other(format!(
                    "extracted package file '{}' byte count overflowed",
                    entry.path().display()
                ))
            })?;
            content_hasher.update(&buffer[..read]);
        }
        if actual_size != metadata.len() {
            return Ok(false);
        }
        files.push((
            path_to_signature_string(rel),
            actual_size,
            metadata_is_executable(&metadata),
            hex::encode(content_hasher.finalize()),
        ));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut hasher = Sha256::new();
    let mut unpacked_size = 0u64;
    for (path, size, executable, digest) in &files {
        update_tree_hasher(&mut hasher, path, *size, *executable, digest);
        unpacked_size = unpacked_size
            .checked_add(*size)
            .ok_or_else(|| MgError::Other("extracted package size overflowed".to_string()))?;
    }

    Ok(expected.file_count == files.len() as u64
        && expected.unpacked_size == unpacked_size
        && expected.file_tree_sha256 == hex::encode(hasher.finalize()))
}
