//! Avoid rewriting files when their current bytes already match.
//! Tránh ghi lại file khi byte hiện tại đã giống nhau.

use crate::manifest::atomic_write::atomic_write;
use crate::manifest::error::ManifestError;
use mgc_types::MgResult;
use std::path::Path;

/// Return `false` for identical bytes, otherwise atomically write and return `true`.
/// Trả `false` nếu byte giống nhau; nếu khác thì ghi atomic và trả `true`.
pub fn atomic_write_if_changed(path: &Path, data: &[u8]) -> MgResult<bool> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => {
            let existing = mgc_adapter_base::project_file::read_regular_text(path, "package.json")?;
            if existing.as_bytes() == data {
                return Ok(false);
            }
        }
        Ok(_) => {
            return Err(ManifestError::UnsafeUpdateDestination {
                path: path.to_path_buf(),
            }
            .into());
        }
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(ManifestError::InspectUpdateDestination {
                path: path.to_path_buf(),
                source,
            }
            .into());
        }
    }

    atomic_write(path, data)?;
    Ok(true)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
