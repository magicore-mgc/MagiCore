//! Syncs a renamed file's parent directory where the platform supports it.
//! Đồng bộ thư mục cha sau rename trên platform hỗ trợ.

use std::path::Path;

#[cfg(unix)]
use super::super::error::{CasWriteOperation, map_io_error};
use crate::cas::store::StoreError;

/// Sync the parent directory of a just-renamed file so the rename itself is
/// crash-durable (audit vòng-3 P1-7: fsync on the FILE without a directory
/// fsync leaves the rename un-durable across power loss). Unix only; on
/// other platforms this is a documented no-op (Windows NTFS journals
/// metadata differently).
///
/// Đồng bộ thư mục cha của file vừa rename để chính rename được bền vững
/// khi crash (P1-7 audit vòng-3: fsync FILE mà không fsync thư mục thì
/// rename chưa bền qua mất điện). Chỉ Unix; platform khác là no-op được
/// ghi rõ (Windows NTFS journal metadata theo cách khác).
pub fn sync_parent_dir(dir: &Path) -> Result<(), StoreError> {
    #[cfg(unix)]
    {
        std::fs::File::open(dir)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| map_io_error(dir, CasWriteOperation::ParentDirectorySync, error))?;
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
    }
    Ok(())
}
