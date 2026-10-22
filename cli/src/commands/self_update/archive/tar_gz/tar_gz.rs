//! Read a regular binary from tar.gz.
//! Đọc binary thường từ tar.gz.

use crate::commands::self_update::archive::set_executable;
use anyhow::Result;
/// Pick the `mgc`/`mgc.exe` REGULAR file out of a tar.gz.
/// (Chọn file regular mgc trong tar.gz.)
pub(in crate::commands::self_update) fn stage_from_tar_gz(
    archive_path: &std::path::Path,
    staging_path: &std::path::Path,
) -> Result<Option<std::path::PathBuf>> {
    let file = std::fs::File::open(archive_path)
        .map_err(|e| anyhow::anyhow!("open {}: {e}", archive_path.display()))?;
    let gz = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(gz);
    for entry in archive
        .entries()
        .map_err(|e| anyhow::anyhow!("read tar {}: {e}", archive_path.display()))?
    {
        let mut entry = entry.map_err(|e| anyhow::anyhow!("read tar entry: {e}"))?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path = entry
            .path()
            .map_err(|e| anyhow::anyhow!("read tar entry path: {e}"))?;
        let Some(file_name) = path.file_name() else {
            continue;
        };
        if file_name == "mgc" || file_name == "mgc.exe" {
            let dest = staging_path.join(file_name);
            entry
                .unpack(&dest)
                .map_err(|e| anyhow::anyhow!("extract {}: {e}", dest.display()))?;
            set_executable(&dest)?;
            return Ok(Some(dest));
        }
    }
    Ok(None)
}
