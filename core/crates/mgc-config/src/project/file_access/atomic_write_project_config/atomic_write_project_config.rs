//! Publish project configuration through a unique same-directory stage file.
//! Công bố cấu hình project qua file staging duy nhất cùng thư mục.
use std::io::Write;
use std::path::Path;

use super::super::error::{FileAccessError, Result};
use super::super::project_config_metadata::ensure_regular_project_config_typed;

/// Preserve the existing anyhow boundary used by project identity writers.
/// Giữ biên anyhow hiện tại của writer project identity.
pub(in crate::project::project) fn atomic_write_project_config(
    path: &Path,
    bytes: &[u8],
) -> anyhow::Result<()> {
    atomic_write_project_config_typed(path, bytes).map_err(FileAccessError::into_anyhow)
}

fn atomic_write_project_config_typed(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path.file_name().ok_or(FileAccessError::MissingFileName)?;
    let metadata = ensure_regular_project_config_typed(path)?;
    let temp_path = parent.join(format!(
        ".{}.tmp.{}.{}",
        file_name.to_string_lossy(),
        std::process::id(),
        uuid::Uuid::new_v4()
    ));

    let result = (|| -> Result<()> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp_path)?;
        if let Some(metadata) = &metadata {
            file.set_permissions(metadata.permissions())?;
        }
        file.write_all(bytes)?;
        file.sync_all()?;
        ensure_regular_project_config_typed(path)?;
        mgc_lockfile::atomic::atomic_replace_file(&temp_path, path)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp_path);
    }
    result
}
