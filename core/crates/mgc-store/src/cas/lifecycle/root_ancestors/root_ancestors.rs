//! Validate existing ancestors of the CAS root against redirection.
//! Kiểm tra ancestor hiện có của root CAS để chặn chuyển hướng.
use std::fs;
use std::path::Path;

use crate::cas::store::StoreError;

/// Walk up from the CAS root: every EXISTING ancestor must be a real
/// directory — with ONE narrow, explicit exemption: system-managed
/// symlinks owned by root (uid 0). macOS ships `/var → private/var` and
/// many Linuxes symlink `/tmp` or `/etc/mtab`; these are root-owned, so
/// a same-user attacker (the cache-poisoning threat model) can neither
/// create nor retarget them — refusing them bricks every standard
/// tempdir (REVIEW vòng-2 finding: tempfile::tempdir() on macOS lives
/// under /var). A symlink owned by anyone ELSE — including the current
/// user — stays a hard reject. `symlink_metadata` errors other than
/// NotFound fail closed (P0-B discipline — a permission-denied ancestor
/// must not be skipped).
///
/// Duyệt lên từ gốc CAS: mọi ancestor ĐANG TỒN TẠI phải là thư mục thật
/// — với MỘT ngoại lệ hẹp, tường minh: symlink hệ thống do root (uid 0)
/// sở hữu. macOS ship `/var → private/var` và nhiều bản Linux symlink
/// `/tmp` hay `/etc/mtab`; chúng do root sở hữu nên attacker cùng-user
/// (threat model đầu độc cache) không thể tạo lẫn đổi hướng — từ chối
/// chúng brick mọi tempdir chuẩn (phát hiện REVIEW vòng-2: tempfile::
/// tempdir() trên macOS nằm dưới /var). Symlink do BẤT KỲ ai khác sở hữu
/// — kể cả user hiện tại — vẫn bị chặn tuyệt đối. Lỗi `symlink_metadata`
/// ngoài NotFound fail cứng (kỷ luật P0-B — ancestor bị từ chối quyền
/// không được bỏ qua).
pub(in crate::cas::lifecycle) fn check_cas_root_ancestors(root: &Path) -> Result<(), StoreError> {
    let mut current = root.parent();
    while let Some(p) = current {
        match fs::symlink_metadata(p) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    // Root-owned symlink → system-managed (e.g. /var on
                    // macOS): trusted for the same-user threat model —
                    // the attacker in scope cannot write a root-owned
                    // link. Everything else is a redirect we refuse.
                    // (Symlink do root sở hữu → hệ thống quản (vd /var
                    // trên macOS): tin cậy cho threat model cùng-user —
                    // attacker trong scope không ghi được link root-owned.
                    // Mọi thứ khác là redirect bị từ chối.)
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::MetadataExt;
                        if meta.uid() == 0 {
                            current = p.parent();
                            continue;
                        }
                    }
                    return Err(StoreError::Io {
                        path: p.to_path_buf(),
                        msg: "CAS root ancestor is a symlink".to_string(),
                    });
                }
                if !meta.is_dir() {
                    return Err(StoreError::Io {
                        path: p.to_path_buf(),
                        msg: "CAS root ancestor is not a directory".to_string(),
                    });
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // An absent ancestor is fine — the nearest existing one is
                // the last link in the chain; above it, this branch stops
                // applying because we keep walking and will hit NotFound
                // again only above a symlink-created gap we refuse anyway.
                // (Ancestor chưa tồn tại là hợp lệ — ancestor tồn tại gần
                // nhất là mắt xích cuối; phía trên nhánh này không áp dụng
                // vì duyệt tiếp chỉ gặp NotFound nữa.)
            }
            Err(e) => {
                // PermissionDenied / I/O error — fail closed.
                // (Từ chối quyền / lỗi I/O — fail cứng.)
                return Err(StoreError::Io {
                    path: p.to_path_buf(),
                    msg: format!("CAS root ancestor metadata unreadable: {e}"),
                });
            }
        }
        current = p.parent();
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
