//! Restrict CAS root and internal directory permissions.
//! Giới hạn permission của root CAS và thư mục nội bộ.
use std::fs;
use std::path::Path;

use crate::cas::store::StoreError;

/// Tighten the CAS root + its store subdirs to owner-only (P0-D): the CAS
/// contains package bytes, compiled JS and inline sourcemaps (potentially
/// proprietary source) — world/group-readable defaults leak them to every
/// other local user. 0700 on dirs; blob/record files are set 0600 by the
/// write primitives (write.rs) at creation time.
///
/// Siết permission gốc CAS + các thư mục con store về chỉ-owner (P0-D):
/// CAS chứa bytes package, JS đã biên dịch và sourcemap inline (nguồn
/// độc quyền tiềm năng) — mặc định world-readable làm lộ cho mọi user
/// local khác. 0700 cho thư mục; blob/record được write.rs đặt 0600 ngay
/// lúc tạo.
pub fn set_cas_root_permissions(root: &Path) -> Result<(), StoreError> {
    if !root.exists() {
        return Ok(());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // Owner-only: rwx for the current user, nothing for group/others.
        // Shared multi-user stores need an explicit service boundary, not
        // a world-readable default (Tech Lead P0-D, 2026-09-13).
        // (Chỉ-owner: rwx cho user hiện tại, không gì cho group/others.
        // Store chia sẻ multi-user cần ranh giới service tường minh, không
        // phải mặc định world-readable.)
        let perms = fs::Permissions::from_mode(0o700);
        fs::set_permissions(root, perms.clone()).map_err(|e| StoreError::Io {
            path: root.to_path_buf(),
            msg: e.to_string(),
        })?;
        // Store-internal subdirs: same owner-only mode.
        // (Thư mục con bên trong store: cùng mode chỉ-owner.)
        for dir in [
            root.join("files"),
            root.join("files").join("blake3"),
            root.join("compiled"),
            root.join("compiled").join("blake3"),
            root.join("tmp"),
        ] {
            if dir.exists() {
                fs::set_permissions(&dir, perms.clone()).map_err(|e| StoreError::Io {
                    path: dir,
                    msg: e.to_string(),
                })?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
