//! Create a private staging directory and choose a format-specific reader.
//! Tạo staging riêng và chọn reader theo định dạng.

use super::{stage_from_tar_gz, stage_from_zip};
use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;
/// Extract the single `mgc`/`mgc.exe` binary from the archive into a
/// fresh temp dir. Rejects path traversal inside the archive (zip-slip /
/// tar-slip): entries escaping the staging dir fail closed.
/// (Bung binary khỏi archive — chặn path traversal trong archive.)
pub(in crate::commands) fn stage_binary(
    archive_path: &std::path::Path,
    ext: &str,
) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    let staging = tempfile::tempdir().map_err(|e| anyhow::anyhow!("create staging dir: {e}"))?;
    // The staging dir outlives this function (caller cleans it after a
    // successful swap AND on every failure path).
    // (Staging dir sống qua function — caller dọn mọi đường.)
    let staging_path = staging.keep();
    let found = if ext == "zip" {
        stage_from_zip(archive_path, &staging_path)?
    } else {
        stage_from_tar_gz(archive_path, &staging_path)?
    };
    match found {
        Some(binary) => Ok((staging_path, binary)),
        None => Err(SelfUpdateError::no_binary(
            &archive_path.display().to_string(),
        )),
    }
}
