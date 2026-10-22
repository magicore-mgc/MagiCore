//! Export Permissions owner.
//! Phụ trách export permissions trong luồng export CAS.

use std::fs;
use std::path::Path;

use crate::cas::store::StoreError;

/// Set permissions on an exported (project-side) file. Exported files are
/// ordinary project files — never a hardlink into the CAS.
/// Đặt permission cho file đã xuất ra project. File export là file project
/// bình thường — không bao giờ là hardlink vào CAS.
pub(in crate::cas::store::export) fn set_exported_permissions(
    path: &Path,
    executable: bool,
) -> Result<(), StoreError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = if executable { 0o755 } else { 0o644 };
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(|e| {
            StoreError::Io {
                path: path.to_path_buf(),
                msg: format!("set permissions failed: {e}"),
            }
        })?;
    }
    #[cfg(not(unix))]
    {
        let _ = (path, executable);
    }
    Ok(())
}
