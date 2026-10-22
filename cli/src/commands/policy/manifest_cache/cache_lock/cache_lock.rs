//! Acquire and validate the operating-system cache writer lock.
//! Lấy và xác minh khóa writer cache của hệ điều hành.

use std::path::Path;

use super::super::error::{PolicyManifestCacheError, PolicyManifestCacheResult};

/// OS-backed lock for the shared policy cache; the lock file is retained so
/// concurrent writers always lock the same inode.
/// (Khóa OS cho cache policy dùng chung; giữ file để writer cùng khóa một inode.)
pub(in crate::commands::policy) fn lock_policy_manifest_cache(
    cache_path: &Path,
) -> PolicyManifestCacheResult<std::fs::File> {
    use std::fs::OpenOptions;

    let lock_path = cache_path.with_extension("lock");
    let display_path = lock_path.display().to_string();
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(0x0020_0000);
    }
    let file = options
        .open(&lock_path)
        .map_err(|source| PolicyManifestCacheError::LockOpen {
            path: display_path.clone(),
            source,
        })?;
    let metadata = std::fs::symlink_metadata(&lock_path).map_err(|source| {
        PolicyManifestCacheError::LockPathInspect {
            path: display_path.clone(),
            source,
        }
    })?;
    let open_metadata =
        file.metadata()
            .map_err(|source| PolicyManifestCacheError::LockFileInspect {
                path: display_path.clone(),
                source,
            })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || !open_metadata.is_file() {
        return Err(PolicyManifestCacheError::LockPathNotRegular);
    }
    fs2::FileExt::lock_exclusive(&file).map_err(|source| {
        PolicyManifestCacheError::LockAcquire {
            path: display_path,
            source,
        }
    })?;
    Ok(file)
}

#[cfg(test)]
#[path = "test/cache_lock.rs"]
mod tests;
