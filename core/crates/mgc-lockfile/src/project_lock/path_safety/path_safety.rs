//! Checks the project guard directory and rejects symlink/reparse paths.
//! Kiểm tra directory guard project và từ chối path symlink/reparse.

use std::path::Path;

use super::error::PathSafetyError;

/// Ensure `.magicore` is a real directory, creating only this single level.
/// Đảm bảo `.magicore` là directory thật, chỉ tạo đúng một cấp.
pub(in crate::project_lock) fn ensure_real_dir(dir: &Path) -> Result<(), PathSafetyError> {
    match std::fs::symlink_metadata(dir) {
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => std::fs::create_dir(dir)
            .map_err(|source| PathSafetyError::CreateDirectory {
                path: dir.display().to_string(),
                source,
            }),
        Err(source) => Err(PathSafetyError::StatDirectory {
            path: dir.display().to_string(),
            source,
        }),
        Ok(meta) => {
            if path_is_link_or_reparse(dir) {
                return Err(PathSafetyError::LinkSwappedDirectory {
                    path: dir.display().to_string(),
                });
            }
            if !meta.file_type().is_dir() {
                return Err(PathSafetyError::NotDirectory {
                    path: dir.display().to_string(),
                });
            }
            Ok(())
        }
    }
}

/// Refuse a symlink/reparse guard; preserve the existing missing/error fallback.
/// Từ chối guard symlink/reparse; giữ fallback hiện tại khi thiếu/lỗi metadata.
pub(in crate::project_lock) fn refuse_link_swap(guard_path: &Path) -> Result<(), PathSafetyError> {
    let Ok(meta) = std::fs::symlink_metadata(guard_path) else {
        return Ok(());
    };
    if meta.file_type().is_symlink() {
        return Err(PathSafetyError::SymlinkGuard {
            path: guard_path.display().to_string(),
        });
    }
    #[cfg(windows)]
    {
        if super::windows_reparse::windows_reparse_point(guard_path) {
            return Err(PathSafetyError::ReparseGuard {
                path: guard_path.display().to_string(),
            });
        }
    }
    Ok(())
}

/// True for symlinks on all platforms and Windows reparse points.
/// True với symlink trên mọi platform và reparse point trên Windows.
pub fn path_is_link_or_reparse(path: &Path) -> bool {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return false;
    };
    if meta.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        super::windows_reparse::windows_reparse_point(path)
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
