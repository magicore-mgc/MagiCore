//! Set the staged binary executable on supported operating systems.
//! Đặt quyền chạy cho binary staging theo hệ điều hành.

use anyhow::Result;
#[cfg(unix)]
pub(in crate::commands::self_update) fn set_executable(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let meta =
        std::fs::metadata(path).map_err(|e| anyhow::anyhow!("stat {}: {e}", path.display()))?;
    let mut perms = meta.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)
        .map_err(|e| anyhow::anyhow!("chmod {}: {e}", path.display()))
}

#[cfg(not(unix))]
pub(in crate::commands::self_update) fn set_executable(_path: &std::path::Path) -> Result<()> {
    Ok(())
}
