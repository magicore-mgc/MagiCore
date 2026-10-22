//! Validate a present project lock without treating it as core identity.
//! Kiểm tra lock project nhưng không xem lock là identity của một core.

use anyhow::Result;
use mgc_config::project::read_regular_project_text;
use std::path::Path;

pub(in crate::dispatch) fn validate(root: &Path) -> Result<()> {
    let lock_path = root.join("mgc.lock");
    if let Some(content) = read_regular_project_text(&lock_path, "lockfile")? {
        let _: toml::Value = toml::from_str(&content)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
