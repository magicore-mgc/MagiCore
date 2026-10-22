//! Reads regular package manifests with size and no-follow guards.
//! Đọc package manifest dạng file thường với giới hạn dung lượng và chặn follow symlink.

use std::io::Read;
use std::path::Path;

use super::super::super::{PendingResult, TrustPendingError};

// Match the established Web manifest reader bound — giữ cùng giới hạn của bộ đọc manifest Web.
const MAX_TRUST_PACKAGE_JSON_BYTES: u64 = 10 * 1024 * 1024;

pub(in crate::commands::trust::pending::package_scan) fn read_bounded_package_json(
    path: &Path,
) -> PendingResult<Option<Vec<u8>>> {
    let path_kind = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(TrustPendingError::scan_failed(path, error)),
    };
    if !path_kind.file_type().is_file() {
        return Err(TrustPendingError::manifest_invalid(
            path,
            "must be a regular non-symlink file",
        ));
    }
    if path_kind.len() > MAX_TRUST_PACKAGE_JSON_BYTES {
        return Err(TrustPendingError::manifest_too_large(
            path,
            MAX_TRUST_PACKAGE_JSON_BYTES,
        ));
    }

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // OPEN_REPARSE_POINT prevents following a swapped-in symlink/reparse point.
        options.custom_flags(0x0020_0000);
    }
    let file = options
        .open(path)
        .map_err(|error| TrustPendingError::scan_failed(path, error))?;
    let metadata = file
        .metadata()
        .map_err(|error| TrustPendingError::scan_failed(path, error))?;
    if !metadata.is_file() {
        return Err(TrustPendingError::manifest_invalid(
            path,
            "must be a regular non-symlink file",
        ));
    }
    if metadata.len() > MAX_TRUST_PACKAGE_JSON_BYTES {
        return Err(TrustPendingError::manifest_too_large(
            path,
            MAX_TRUST_PACKAGE_JSON_BYTES,
        ));
    }

    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(MAX_TRUST_PACKAGE_JSON_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| TrustPendingError::scan_failed(path, error))?;
    if bytes.len() as u64 > MAX_TRUST_PACKAGE_JSON_BYTES {
        return Err(TrustPendingError::manifest_too_large(
            path,
            MAX_TRUST_PACKAGE_JSON_BYTES,
        ));
    }
    Ok(Some(bytes))
}

#[cfg(test)]
#[path = "test/bounded_manifest.rs"]
mod tests;
