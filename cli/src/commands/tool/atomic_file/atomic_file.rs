use super::super::json_file::write_new_private_file;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub(in crate::commands::tool) fn atomic_replace_file(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("managed file '{}' has no parent", path.display()))?;
    let temp = parent.join(format!(
        ".mgc-tool-{}-{}.tmp",
        std::process::id(),
        uuid_suffix()
    ));
    write_new_private_file(&temp, contents)?;
    #[cfg(windows)]
    if path.exists() {
        let backup = path.with_extension(format!("mgc-old-{}", uuid_suffix()));
        fs::rename(path, &backup)?;
        if let Err(error) = fs::rename(&temp, path) {
            let _ = fs::rename(&backup, path);
            let _ = fs::remove_file(&temp);
            return Err(error.into());
        }
        fs::remove_file(backup)?;
        return Ok(());
    }
    fs::rename(&temp, path)?;
    #[cfg(unix)]
    fs::File::open(parent)?.sync_all()?;
    Ok(())
}
fn uuid_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
