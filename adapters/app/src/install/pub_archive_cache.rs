//! Verified archive cache for native Pub lock replay.
//! Giữ archive Pub theo checksum để replay native không cần registry.

use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

const ARCHIVE_DIRECTORY: &str = "archives";
const DEFAULT_ARCHIVE_MAX_BYTES: u64 = 256 * 1024 * 1024;
const ARCHIVE_MAX_BYTES_ENV: &str = "MGC_PUB_ARCHIVE_MAX_BYTES";

fn archive_limit() -> MgResult<u64> {
    match std::env::var(ARCHIVE_MAX_BYTES_ENV) {
        Ok(value) => value
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .ok_or_else(|| {
                MgError::Other(format!(
                    "{ARCHIVE_MAX_BYTES_ENV} must be a positive integer"
                ))
            }),
        Err(std::env::VarError::NotPresent) => Ok(DEFAULT_ARCHIVE_MAX_BYTES),
        Err(error) => Err(MgError::Other(format!(
            "read {ARCHIVE_MAX_BYTES_ENV}: {error}"
        ))),
    }
}

fn archive_path(root: &Path, expected: &str) -> MgResult<PathBuf> {
    if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(MgError::Integrity(
            "Pub archive cache requires a valid SHA-256 digest".into(),
        ));
    }
    let directory = root.join(ARCHIVE_DIRECTORY);
    match std::fs::symlink_metadata(&directory) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => {
            return Err(MgError::Integrity(
                "refusing linked or non-directory Pub archive cache".into(),
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(MgError::Other(format!(
                "inspect Pub archive cache: {error}"
            )));
        }
    }
    Ok(directory.join(format!("{}.tar.gz", expected.to_ascii_lowercase())))
}

pub(super) fn load_verified(root: &Path, expected: &str) -> MgResult<Option<Vec<u8>>> {
    load_with_limit(root, expected, archive_limit()?)
}

fn load_with_limit(root: &Path, expected: &str, limit: u64) -> MgResult<Option<Vec<u8>>> {
    let path = archive_path(root, expected)?;
    match std::fs::symlink_metadata(&path) {
        Ok(meta) if meta.file_type().is_symlink() || !meta.is_file() => {
            return Err(MgError::Integrity(
                "refusing linked or non-file Pub archive".into(),
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(MgError::Other(format!("inspect Pub archive: {error}"))),
    }
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    // Recheck at open on Unix so a swapped symlink cannot redirect the read.
    // Unix mở no-follow để symlink thay giữa metadata và open không đổi đích đọc.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options
        .open(&path)
        .map_err(|error| MgError::Other(format!("open Pub archive: {error}")))?;
    // Validate the opened object, not only the earlier pathname metadata.
    // Kiểm tra handle đã mở để tránh đọc object khác nếu path đổi giữa chừng.
    if !file
        .metadata()
        .map_err(|error| MgError::Other(format!("inspect opened Pub archive: {error}")))?
        .is_file()
    {
        return Err(MgError::Integrity(
            "opened Pub archive is not a regular file".into(),
        ));
    }
    let mut bytes = Vec::new();
    file.take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| MgError::Other(format!("read Pub archive: {error}")))?;
    if bytes.len() as u64 > limit {
        return Err(MgError::Integrity(format!(
            "Pub archive exceeds {ARCHIVE_MAX_BYTES_ENV} limit"
        )));
    }
    verify(expected, &bytes)?;
    Ok(Some(bytes))
}

fn verify(expected: &str, bytes: &[u8]) -> MgResult<()> {
    if !hex::encode(Sha256::digest(bytes)).eq_ignore_ascii_case(expected) {
        return Err(MgError::Integrity(
            "cached Pub archive SHA-256 mismatch".into(),
        ));
    }
    Ok(())
}

pub(super) fn save_verified(root: &Path, expected: &str, bytes: &[u8]) -> MgResult<()> {
    let path = archive_path(root, expected)?;
    verify(expected, bytes)?;
    if bytes.len() as u64 > archive_limit()? {
        return Err(MgError::Integrity(format!(
            "Pub archive exceeds {ARCHIVE_MAX_BYTES_ENV} limit"
        )));
    }
    let directory = path
        .parent()
        .ok_or_else(|| MgError::Other("Pub archive cache has no parent".into()))?;
    std::fs::create_dir_all(directory)
        .map_err(|error| MgError::Other(format!("create Pub archive cache: {error}")))?;
    super::atomic_write_project_file(&path, bytes, "Pub archive")
}

#[cfg(test)]
#[path = "test/pub_archive_cache.rs"]
mod tests;
