use anyhow::Result;
use std::path::PathBuf;

pub(in crate::commands::config) fn npmrc_path(local: bool) -> Result<PathBuf> {
    if local {
        return Ok(std::env::current_dir()?.join(".npmrc"));
    }
    Ok(dirs::home_dir()
        .ok_or_else(crate::error::no_home_dir)?
        .join(".npmrc"))
}

/// Tìm mgc.toml project root từ CWD leo lên parent (giống find_project_root)
pub(in crate::commands::config) fn find_mgc_toml() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let candidate = dir.join("mgc.toml");
        if candidate.exists() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}
