//! Link-safe project reads: refuse swapped paths and open with O_NOFOLLOW.
//! Đọc project an toàn với link: từ chối path bị tráo và mở với O_NOFOLLOW.

use std::path::Path;

use anyhow::Result;

/// Refuse symlink/junction/reparse-point project paths: a swapped path
/// would move a read or write outside the project (shares the rule with
/// the writer-lock guard).
/// (Từ chối symlink/junction/reparse — chống path bị tráo.)
pub(crate) fn refuse_project_link(path: &Path) -> Result<()> {
    if mgc_lockfile::project_lock::path_is_link_or_reparse(path) {
        return Err(anyhow::anyhow!(
            "refusing link-swapped project path '{}'",
            path.display()
        ));
    }
    Ok(())
}

/// Open a file for reading with O_NOFOLLOW (unix): a planted
/// final-component symlink fails the open instead of redirecting.
/// (Mở đọc có O_NOFOLLOW.)
#[cfg(unix)]
fn open_nofollow_read(path: &Path) -> Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|e| anyhow::anyhow!("cannot open '{}': {e:#}", path.display()))
}

pub(crate) fn read_project_string(path: &Path) -> Result<String> {
    refuse_project_link(path)?;
    read_project_text(path)
}

#[cfg(unix)]
fn read_project_text(path: &Path) -> Result<String> {
    use std::io::Read;
    let mut content = String::new();
    open_nofollow_read(path)?
        .read_to_string(&mut content)
        .map_err(|e| anyhow::anyhow!("cannot read '{}': {e:#}", path.display()))?;
    Ok(content)
}

#[cfg(not(unix))]
fn read_project_text(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("cannot read '{}': {e:#}", path.display()))
}
