//! Stage and atomically replace regular project files.
//! Tạo staging và thay file project thường bằng thao tác atomic.

use super::super::{
    error::{error, error_with_source},
    regular_file::reject_non_regular,
};
use mgc_types::MgError;
use std::path::{Path, PathBuf};

/// Replace an existing regular manifest atomically from a unique staging
/// file on the same filesystem. The destination symlink (if swapped after
/// validation) is replaced as a directory entry, never followed as a target.
/// Thay entry đích bằng rename atomic cùng filesystem; không mở symlink để ghi vào target.
pub(in crate::project_file) fn atomic_write_regular(
    path: &Path,
    bytes: &[u8],
) -> Result<(), MgError> {
    use std::io::Write;

    let original_metadata = reject_non_regular(path)?;
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .ok_or_else(|| error(format!("invalid manifest path: {}", path.display())))?;
    let temp = unique_temp_path(parent, file_name);
    let result = (|| {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp).map_err(|source| {
            error_with_source(source, |e| format!("create manifest staging file: {e}"))
        })?;
        file.set_permissions(original_metadata.permissions())
            .map_err(|source| {
                error_with_source(source, |e| format!("preserve manifest permissions: {e}"))
            })?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|source| {
                error_with_source(source, |e| format!("sync manifest staging file: {e}"))
            })?;
        // Revalidate path immediately before atomic replacement. The OS
        // replacement primitive replaces the path entry; it does not open
        // the old file for truncation.
        // Kiểm tra lại trước rename; thao tác thay entry không mở file cũ để truncate.
        reject_non_regular(path)?;
        mgc_lockfile::atomic::atomic_replace_file(&temp, path).map_err(|source| {
            error_with_source(source, |e| {
                format!("replace manifest {}: {e}", path.display())
            })
        })?;
        #[cfg(unix)]
        std::fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|source| {
                error_with_source(source, |e| format!("sync manifest directory: {e}"))
            })?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

fn unique_temp_path(parent: &Path, file_name: &std::ffi::OsStr) -> PathBuf {
    parent.join(format!(
        ".{}.mgc-tmp-{}-{}",
        file_name.to_string_lossy(),
        std::process::id(),
        uuid::Uuid::new_v4()
    ))
}

#[cfg(test)]
#[path = "test/atomic_file.rs"]
mod tests;
