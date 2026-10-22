//! Reads bounded regular files without following the final link/reparse point.
//! Đọc file thường có giới hạn, không follow link/reparse cuối.

use std::io::Read;
use std::path::Path;

use mgc_platform::{OpenRegularFileError, open_regular_file_no_follow};

use crate::LockfileResult;

use super::error::BoundedReadError;

const MAX_LOCKFILE_SIZE: u64 = 10 * 1024 * 1024;
const MAX_SIGNATURE_FILE_SIZE: u64 = 64 * 1024;

/// Read a regular file without following links, bounded by `max_size`.
/// Đọc file thường, không follow link, giới hạn ở `max_size` byte.
pub fn read_bounded_regular_file(
    path: &Path,
    max_size: u64,
    label: &str,
) -> LockfileResult<Vec<u8>> {
    read_bounded_regular_file_inner(path, max_size, label).map_err(Into::into)
}

fn read_bounded_regular_file_inner(
    path: &Path,
    max_size: u64,
    label: &str,
) -> Result<Vec<u8>, BoundedReadError> {
    let read_limit = max_size
        .checked_add(1)
        .ok_or_else(|| BoundedReadError::LimitOverflow {
            label: label.to_string(),
        })?;
    let max_capacity = usize::try_from(max_size)
        .ok()
        .filter(|capacity| *capacity < usize::MAX)
        .ok_or_else(|| BoundedReadError::LimitExceedsAddressSpace {
            label: label.to_string(),
        })?;
    let path_metadata = std::fs::symlink_metadata(path)?;
    if path_metadata.file_type().is_symlink() || !path_metadata.is_file() {
        return Err(BoundedReadError::NotRegularPath {
            label: label.to_string(),
            path: path.display().to_string(),
        });
    }
    if path_metadata.len() > max_size {
        return Err(BoundedReadError::PathTooLarge {
            label: label.to_string(),
            size: path_metadata.len(),
            max: max_size,
        });
    }

    let file = open_regular_file_no_follow(path).map_err(|error| match error {
        OpenRegularFileError::Symlink | OpenRegularFileError::NotRegular => {
            BoundedReadError::NotRegularPath {
                label: label.to_string(),
                path: path.display().to_string(),
            }
        }
        OpenRegularFileError::OpenedNotRegular => BoundedReadError::OpenedNotRegular {
            label: label.to_string(),
            path: path.display().to_string(),
        },
        #[cfg(windows)]
        OpenRegularFileError::ReparsePoint => BoundedReadError::ReparsePoint {
            label: label.to_string(),
            path: path.display().to_string(),
        },
        #[cfg(not(any(unix, windows)))]
        OpenRegularFileError::UnsupportedPlatform => BoundedReadError::UnsupportedPlatform {
            label: label.to_string(),
        },
        OpenRegularFileError::Io(source) => BoundedReadError::Io(source),
    })?;
    let opened_metadata = file.metadata()?;
    if !opened_metadata.is_file() {
        return Err(BoundedReadError::OpenedNotRegular {
            label: label.to_string(),
            path: path.display().to_string(),
        });
    }
    if opened_metadata.len() > max_size {
        return Err(BoundedReadError::PathTooLarge {
            label: label.to_string(),
            size: opened_metadata.len(),
            max: max_size,
        });
    }

    let capacity = usize::try_from(opened_metadata.len())
        .ok()
        .filter(|capacity| *capacity <= max_capacity)
        .ok_or_else(|| BoundedReadError::OpenedSizeExceedsAddressSpace {
            label: label.to_string(),
        })?;
    let mut bytes = Vec::with_capacity(capacity);
    file.take(read_limit).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > max_size {
        return Err(BoundedReadError::ReadExceededLimit {
            label: label.to_string(),
            max: max_size,
        });
    }
    Ok(bytes)
}

/// Read canonical `mgc.lock` bytes with the fixed lockfile size cap.
/// Đọc `mgc.lock` canonical với giới hạn kích thước lockfile cố định.
pub fn read_lockfile_bytes(path: &Path) -> LockfileResult<Vec<u8>> {
    read_bounded_regular_file(path, MAX_LOCKFILE_SIZE, "lockfile")
}

/// Read signature sidecar bytes with the smaller signature size cap.
/// Đọc sidecar chữ ký với giới hạn nhỏ hơn dành riêng cho chữ ký.
pub fn read_signature_file_bytes(path: &Path) -> LockfileResult<Vec<u8>> {
    read_bounded_regular_file(path, MAX_SIGNATURE_FILE_SIZE, "signature file")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
