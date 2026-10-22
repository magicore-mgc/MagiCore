//! Verify cached tarball contents against a marked package root and import CAS blobs.
//! Xác minh tarball cache với package root có marker và import blob vào CAS.
use super::super::cached_files::{cached_archive_file_matches, cached_package_files};
use crate::cache::ExtractedPackageMarker;
use crate::install::package_marker::{
    common_tarball_root_prefix, compute_sha256_hex, extracted_marker_has_content_signature,
    mode_is_executable, path_to_signature_string, sanitize_tarball_signature_path,
    update_tree_hasher,
};
use mgc_store::{ContentStore, Database};
use mgc_types::{MgError, MgResult};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::io::Read;
use std::path::{Path, PathBuf};

struct VerifiedCachedArchiveFile {
    archive_path: PathBuf,
    data: std::ops::Range<usize>,
    executable: bool,
    signature_executable: bool,
}

/// Verify a canonical extracted root against its lock-verified archive while
/// importing the archive's blobs into CAS, without rewriting the root.
/// Xác minh root đã extract với archive đã xác minh theo lock và nạp blob vào
/// CAS mà không ghi đè root.
pub fn import_and_verify_cached_root_from_tarball<R: Read>(
    reader: R,
    package_root: &Path,
    expected_marker: &ExtractedPackageMarker,
    store: &ContentStore,
    claim: Option<(&Database, &str, i64)>,
) -> MgResult<bool> {
    let profile = std::env::var_os("MAGICORE_WEB_PROFILE_INSTALL").is_some();
    let verify_started = std::time::Instant::now();
    if !extracted_marker_has_content_signature(expected_marker) {
        return Ok(false);
    }
    let root_metadata = std::fs::symlink_metadata(package_root).map_err(|err| {
        MgError::Other(format!(
            "failed to inspect cached package root '{}': {err}",
            package_root.display()
        ))
    })?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Ok(false);
    }
    let Some(mut cached_files) = cached_package_files(package_root)? else {
        return Ok(false);
    };

    let decoder = flate2::read::GzDecoder::new(reader);
    let mut archive = tar::Archive::new(decoder);
    let mut files = Vec::<VerifiedCachedArchiveFile>::new();
    let mut archive_data = Vec::new();
    let mut archive_paths = HashSet::<PathBuf>::new();

    for entry in archive
        .entries()
        .map_err(|err| MgError::Other(format!("failed to read cached package archive: {err}")))?
    {
        let mut entry = entry
            .map_err(|err| MgError::Other(format!("failed to read cached archive entry: {err}")))?;
        let entry_type = entry.header().entry_type();
        if matches!(entry_type.as_byte(), b'g' | b'x') {
            continue;
        }
        let archive_path = sanitize_tarball_signature_path(
            entry
                .path()
                .map_err(|err| MgError::Other(format!("failed to read archive entry path: {err}")))?
                .as_ref(),
        )?;
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(MgError::Other(format!(
                "tar links are not allowed in cached package archive: {}",
                archive_path.display()
            )));
        }
        if entry_type.is_dir() {
            continue;
        }
        if !entry_type.is_file() {
            return Err(MgError::Other(format!(
                "unsupported tar entry type in cached package archive: {}",
                archive_path.display()
            )));
        }
        if !archive_paths.insert(archive_path.clone()) {
            return Ok(false);
        }

        let size = entry.header().size().map_err(|err| {
            MgError::Other(format!(
                "failed to read cached archive entry size '{}': {err}",
                archive_path.display()
            ))
        })?;
        let mode = entry.header().mode().map_err(|err| {
            MgError::Other(format!(
                "failed to read cached archive entry mode '{}': {err}",
                archive_path.display()
            ))
        })?;
        let signature_executable = mode_is_executable(mode);
        let cas_executable = mode & 0o111 != 0;
        let data_start = archive_data.len();
        entry.read_to_end(&mut archive_data).map_err(|err| {
            MgError::Other(format!(
                "failed to read cached archive entry '{}': {err}",
                archive_path.display()
            ))
        })?;
        let data_end = archive_data.len();
        if (data_end - data_start) as u64 != size {
            return Ok(false);
        }
        files.push(VerifiedCachedArchiveFile {
            archive_path,
            data: data_start..data_end,
            executable: cas_executable,
            signature_executable,
        });
    }

    let archive_paths_and_sizes = files
        .iter()
        .map(|file| {
            (
                path_to_signature_string(&file.archive_path),
                (file.data.end - file.data.start) as u64,
            )
        })
        .collect::<Vec<_>>();
    let root_prefix = common_tarball_root_prefix(&archive_paths_and_sizes);
    let mut files_to_verify = Vec::with_capacity(files.len());
    for file in &files {
        let relative_path = if root_prefix.is_some() {
            file.archive_path.components().skip(1).collect::<PathBuf>()
        } else {
            file.archive_path.clone()
        };
        if relative_path.as_os_str().is_empty() {
            return Ok(false);
        }
        let Some(cached_metadata) = cached_files.remove(&relative_path) else {
            return Ok(false);
        };
        files_to_verify.push((file, relative_path, cached_metadata));
    }
    if !cached_files.is_empty() {
        return Ok(false);
    }
    let verified_signature_files = files_to_verify
        .into_par_iter()
        .map(|(file, relative_path, cached_metadata)| {
            let expected_data = &archive_data[file.data.start..file.data.end];
            if !cached_archive_file_matches(
                package_root,
                &relative_path,
                cached_metadata,
                expected_data,
                file.signature_executable,
            )? {
                return Ok(None);
            }
            Ok(Some((
                path_to_signature_string(&relative_path),
                expected_data.len() as u64,
                file.signature_executable,
                compute_sha256_hex(expected_data),
            )))
        })
        .collect::<Result<Vec<_>, MgError>>()?;
    if verified_signature_files.iter().any(Option::is_none) {
        return Ok(false);
    }
    let mut signature_files = verified_signature_files
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    signature_files.sort_by(|left, right| left.0.cmp(&right.0));
    let mut tree_hasher = Sha256::new();
    let mut unpacked_size = 0u64;
    for (path, size, executable, digest) in &signature_files {
        update_tree_hasher(&mut tree_hasher, path, *size, *executable, digest);
        unpacked_size = unpacked_size
            .checked_add(*size)
            .ok_or_else(|| MgError::Other("cached package size overflowed".to_string()))?;
    }
    if expected_marker.file_count != signature_files.len() as u64
        || expected_marker.unpacked_size != unpacked_size
        || expected_marker.file_tree_sha256 != hex::encode(tree_hasher.finalize())
    {
        return Ok(false);
    }
    let verify_ms = verify_started.elapsed().as_millis();

    let import_started = std::time::Instant::now();
    let hashes = store
        .import_rebuildable_bytes_with_exec_batch(files.iter().map(|file| {
            (
                &archive_data[file.data.start..file.data.end],
                file.executable,
            )
        }))
        .map_err(|err| {
            MgError::Store(format!(
                "failed to import verified cached package into CAS: {err}"
            ))
        })?;
    let cas_import_ms = import_started.elapsed().as_millis();
    mgc_store::failpoint::hit("after-cas-publish");
    let claim_started = std::time::Instant::now();
    if let Some((db, project_root, generation)) = claim {
        let mut imported = hashes
            .iter()
            .map(|hash| hash.as_hex().to_string())
            .collect::<Vec<_>>();
        imported.sort_unstable();
        imported.dedup();
        let hash_refs = imported.iter().map(String::as_str).collect::<Vec<_>>();
        if !hash_refs.is_empty() {
            db.cas_claim_batch(project_root, generation, &hash_refs)
                .map_err(|err| {
                    MgError::Store(format!(
                        "failed to claim verified cached package blobs: {err}"
                    ))
                })?;
            mgc_store::failpoint::hit("after-first-claim");
        }
    }
    if profile {
        eprintln!(
            "[magicore:web:warm-verify-profile] package_files={} unpacked_bytes={} verify_ms={verify_ms} cas_import_ms={cas_import_ms} claim_ms={}",
            signature_files.len(),
            unpacked_size,
            claim_started.elapsed().as_millis()
        );
    }

    Ok(true)
}
