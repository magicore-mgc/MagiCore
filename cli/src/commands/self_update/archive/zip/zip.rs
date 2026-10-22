//! Read a regular binary from ZIP.
//! Đọc binary thường từ ZIP.

use crate::commands::self_update::archive::set_executable;
use anyhow::Result;
/// Pick the `mgc`/`mgc.exe` REGULAR file out of a zip (symlinks,
/// directories, and traversal entries can never match).
/// (Chọn file regular mgc trong zip.)
pub(in crate::commands::self_update) fn stage_from_zip(
    archive_path: &std::path::Path,
    staging_path: &std::path::Path,
) -> Result<Option<std::path::PathBuf>> {
    let file = std::fs::File::open(archive_path)
        .map_err(|e| anyhow::anyhow!("open {}: {e}", archive_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| anyhow::anyhow!("read zip {}: {e}", archive_path.display()))?;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| anyhow::anyhow!("read zip entry {i}: {e}"))?;
        if !entry.is_file() {
            continue;
        }
        let Some(safe_path) = entry.enclosed_name() else {
            continue;
        };
        let file_name = safe_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("zip entry has no file name"))?;
        if file_name == "mgc" || file_name == "mgc.exe" {
            let dest = staging_path.join(file_name);
            let mut out = std::fs::File::create(&dest)
                .map_err(|e| anyhow::anyhow!("create {}: {e}", dest.display()))?;
            std::io::copy(&mut entry, &mut out)
                .map_err(|e| anyhow::anyhow!("extract {}: {e}", dest.display()))?;
            set_executable(&dest)?;
            return Ok(Some(dest));
        }
    }
    Ok(None)
}
