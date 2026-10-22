//! Read a cached Pub archive through bounded, no-follow I/O.
//! Đọc archive Pub qua I/O có giới hạn và không theo symlink.
use mgc_types::{MgError, MgResult};
use std::io::Read;
use std::path::Path;

use super::super::archive_path::archive_path;
use super::super::archive_policy::{ARCHIVE_MAX_BYTES_ENV, archive_limit};
use super::super::archive_verify::verify;

pub(in crate::install::pub_archive_cache) fn load_verified(
    root: &Path,
    expected: &str,
) -> MgResult<Option<Vec<u8>>> {
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

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
