//! Filesystem implementation.
//! Implementation filesystem của shared mutation.

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

/// create_new temp (never overwrite blindly), O_NOFOLLOW on unix, fsync
/// file + parent dir, atomic rename. Mirrors mgc-lockfile atomic semantics.
/// (Ghi file nguyên tử + bền: chống symlink, tmp create_new, fsync.)
pub(crate) fn atomic_write_file(dir: &Path, name: &str, bytes: &[u8]) -> Result<()> {
    atomic_write_file_checked(dir, name, bytes, true)
}

/// Checked variant: `check_dir_link` refuses a swapped parent directory.
/// Journal paths always check; the project root itself may legitimately
/// be a symlinked checkout, so direct project-file writes skip the dir
/// check (the temp file still gets O_NOFOLLOW + create_new, the commit
/// is still an atomic rename).
/// (Biến thể có/không chống link thư mục cha.)
pub(crate) fn atomic_write_file_checked(
    dir: &Path,
    name: &str,
    bytes: &[u8],
    check_dir_link: bool,
) -> Result<()> {
    if check_dir_link {
        refuse_project_link(dir)?;
    }
    let dest = dir.join(name);
    refuse_project_link(&dest)?;
    let tmp = dir.join(format!(
        "{name}.tmp.{}.{}.bak",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0),
    ));
    {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&tmp).map_err(|e| {
            anyhow::anyhow!("cannot create journal temp '{}': {e:#}", tmp.display())
        })?;
        use std::io::Write;
        file.write_all(bytes)
            .map_err(|e| anyhow::anyhow!("cannot write journal temp '{}': {e:#}", tmp.display()))?;
        file.sync_all()
            .map_err(|e| anyhow::anyhow!("cannot fsync journal temp '{}': {e:#}", tmp.display()))?;
    }
    std::fs::rename(&tmp, &dest)?;
    fsync_dir(dir)?;
    Ok(())
}

/// fsync a directory so renames inside it survive power loss.
/// (fsync thư mục — rename bền vững khi mất điện.)
pub(crate) fn fsync_dir(dir: &Path) -> Result<()> {
    match std::fs::File::open(dir) {
        Ok(handle) => handle
            .sync_all()
            .map_err(|e| anyhow::anyhow!("cannot fsync dir '{}': {e:#}", dir.display())),
        // Windows cannot open directories: durability there relies on
        // file fsync + atomic rename; refusal still applies.
        // (Windows không mở được thư mục — bền vững nhờ fsync file.)
        Err(_) => Ok(()),
    }
}
