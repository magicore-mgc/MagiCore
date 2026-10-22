//! Normalize a common package root and hash sorted archive files.
//! Chuẩn hóa package root chung và hash các file archive đã sắp xếp.

use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};

use crate::cache::TarballContentSignature;
use crate::install::package_marker::update_tree_hasher;

use super::super::archive_path::common_tarball_root_prefix;
use super::super::entry_scan::TarballFileSignature;

pub(in crate::install::package_marker::archive_signature) fn signature_from_files(
    files: Vec<TarballFileSignature>,
) -> MgResult<TarballContentSignature> {
    let paths_and_sizes = files
        .iter()
        .map(|(path, size, _, _)| (path.clone(), *size))
        .collect::<Vec<_>>();
    let root_prefix = common_tarball_root_prefix(&paths_and_sizes);
    let mut normalized = files
        .into_iter()
        .filter_map(|(path, size, executable, digest)| {
            let stripped = root_prefix
                .as_ref()
                .and_then(|prefix| {
                    path.strip_prefix(prefix)
                        .and_then(|value| value.strip_prefix('/'))
                })
                .unwrap_or(path.as_str());
            if stripped.is_empty() {
                None
            } else {
                Some((stripped.to_string(), size, executable, digest))
            }
        })
        .collect::<Vec<_>>();
    normalized.sort_by(|left, right| left.0.cmp(&right.0));

    let mut hasher = Sha256::new();
    let mut unpacked_size = 0u64;
    for (path, size, executable, digest) in &normalized {
        update_tree_hasher(&mut hasher, path, *size, *executable, digest);
        unpacked_size = unpacked_size
            .checked_add(*size)
            .ok_or_else(|| MgError::Other("tarball unpacked size overflowed".to_string()))?;
    }

    Ok(TarballContentSignature {
        file_count: normalized.len() as u64,
        unpacked_size,
        file_tree_sha256: hex::encode(hasher.finalize()),
    })
}
