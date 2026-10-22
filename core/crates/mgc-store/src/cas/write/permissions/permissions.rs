//! Applies private CAS blob permissions with existing platform semantics.
//! Đặt permission riêng tư cho blob CAS, giữ nguyên semantics theo platform.

#[cfg(unix)]
use super::super::error::{CasWriteOperation, map_io_error};
#[cfg(unix)]
use std::fs;
use std::path::Path;

use crate::cas::store::StoreError;

pub(in crate::cas::write) fn set_permissions(
    path: &Path,
    executable: bool,
) -> Result<(), StoreError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // CAS-internal blobs are owner-only (P0-D, Tech Lead vòng-7
        // 2026-09-13): the store holds package bytes, compiled JS and inline
        // sourcemaps — world/group-readable modes leak them to other local
        // users. The executable BIT is still tracked (0700 vs 0600) so the
        // export path can restore the right mode on the project side
        // (set_exported_permissions applies 0755/0644 there).
        // Blob bên trong CAS chỉ-owner (P0-D): store giữ bytes package, JS
        // đã biên dịch và sourcemap inline — mode cho group/others đọc được
        // sẽ lộ chúng cho user local khác. BIT executable vẫn được theo dõi
        // (0700 với 0600) để đường export khôi phục đúng mode phía project
        // (set_exported_permissions áp 0755/0644 ở đó).
        let mode = if executable { 0o700 } else { 0o600 };
        fs::set_permissions(path, fs::Permissions::from_mode(mode))
            .map_err(|error| map_io_error(path, CasWriteOperation::SetPermissions, error))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (path, executable);
    }
    Ok(())
}
