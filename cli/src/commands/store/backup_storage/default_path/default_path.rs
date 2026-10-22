//! Resolves the existing default backup naming convention with a safe final root.
//! Phân giải quy ước tên backup hiện tại và kiểm tra root cuối an toàn.
use super::error::DefaultPathError;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .to_string()
}

pub(in crate::commands::store) fn default_backup_dir() -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir().ok_or(DefaultPathError::NoHomeDirectory)?;
    default_path_under(&home).map_err(anyhow::Error::new)
}

fn default_path_under(home: &Path) -> Result<PathBuf, DefaultPathError> {
    let base = home.join(".magicore");
    match fs::symlink_metadata(&base) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            return Err(DefaultPathError::Symlink(base));
        }
        Ok(metadata) if !metadata.is_dir() => return Err(DefaultPathError::NotDirectory(base)),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => return Err(DefaultPathError::Inspect { path: base, source }),
    }
    Ok(base.join(format!("store_backup_{}", timestamp())))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
