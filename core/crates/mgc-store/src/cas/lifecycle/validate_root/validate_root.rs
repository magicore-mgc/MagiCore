//! Validate CAS root identity, type, ownership, and root symlink status.
//! Kiểm tra danh tính, loại, ownership và symlink của root CAS.
use std::fs;
use std::path::Path;

use super::super::root_ancestors::check_cas_root_ancestors;
use crate::cas::store::StoreError;

/// Validate the CAS root BEFORE the store is used: the root itself and every
/// existing ancestor must be a real directory, never a symlink (P0-A).
/// `fs::metadata()` FOLLOWS symlinks, so the old check compared the metadata
/// of the link TARGET and `is_symlink()` was almost always false — the check
/// never fired for a real symlinked root. `symlink_metadata()` inspects the
/// link itself; walking ancestors catches a symlink planted at any depth.
/// On Windows, junctions/reparse points also surface as symlinks here.
///
/// Validate gốc CAS TRƯỚC khi store dùng: chính root và mọi ancestor đang
/// tồn tại phải là thư mục thật, không bao giờ symlink (P0-A).
/// `fs::metadata()` FOLLOW symlink nên check cũ so metadata của ĐÍCH link,
/// `is_symlink()` gần như luôn false — check không bao giờ kích hoạt với
/// root thực sự là symlink. `symlink_metadata()` soi chính link; duyệt
/// ancestor bắt symlink cắm ở bất kỳ độ sâu nào.
pub fn validate_cas_root(root: &Path) -> Result<(), StoreError> {
    // Fail-closed metadata match (P0-3, adversarial review vòng-9): the
    // old code used `if let Ok(meta)` + `else if !root.exists()` —
    // `Path::exists()` SWALLOWS the metadata error, so PermissionDenied
    // or an I/O failure read as "not there yet" and fell through to the
    // create-then-trust path. The single match below distinguishes
    // NotFound (legit — caller creates) from every other error
    // (permission, I/O — hard failure), and validates the metadata on
    // the success arm.
    // (Match metadata fail-closed (P0-3): code cũ dùng `if let Ok(meta)`
    // + `else if !root.exists()` — `Path::exists()` NUỐT lỗi metadata
    // nên PermissionDenied hay lỗi I/O bị đọc thành "chưa có" rồi rơi
    // qua đường tạo-rồi-tin. Match duy nhất bên dưới phân biệt NotFound
    // (hợp lệ — caller tạo) với mọi lỗi khác (quyền, I/O — fail cứng),
    // và validate metadata trên nhánh thành công.)
    match fs::symlink_metadata(root) {
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                return Err(StoreError::Io {
                    path: root.to_path_buf(),
                    msg: "CAS root is a symlink".to_string(),
                });
            }
            if !meta.is_dir() {
                return Err(StoreError::Io {
                    path: root.to_path_buf(),
                    msg: "CAS root is not a directory".to_string(),
                });
            }
            // Ownership gate (P0-D): a root owned by another user is a
            // trust-boundary violation — they control the content we
            // address. Root (uid 0) is exempt: it is trusted by
            // definition and CI/sudo flows must not brick.
            // (Cổng ownership (P0-D): root do user khác sở hữu là vi
            // phạm ranh giới tin cậy — họ điều khiển nội dung mà ta địa
            // chỉ hóa. Root (uid 0) được miễn: mặc định tin cậy, và luồng
            // CI/sudo không bị brick.)
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let me = current_uid();
                if meta.uid() != me && me != 0 {
                    return Err(StoreError::Io {
                        path: root.to_path_buf(),
                        msg: format!(
                            "CAS root is owned by uid {} (current {}), refusing",
                            meta.uid(),
                            me
                        ),
                    });
                }
            }
        }
        // NotFound is the ONLY benign absence: the caller creates the
        // root below, and the freshly created root is re-validated by
        // ensure_cas_dirs' permission pass. Every OTHER metadata error
        // (permission denied, I/O) fails closed — never mistaken for a
        // missing path.
        // (NotFound là sự vắng mặt LÀNH DUY NHẤT: caller tạo root bên
        // dưới, và root mới tạo được check lại bởi bước permission của
        // ensure_cas_dirs. Mọi lỗi metadata KHÁC (từ chối quyền, I/O)
        // fail cứng — không bao giờ bị nhầm thành path vắng.)
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(StoreError::Io {
                path: root.to_path_buf(),
                msg: format!("CAS root metadata unreadable ({e}) — refusing to guess"),
            });
        }
    }

    // Ancestor sweep (P0-A): a symlink at ANY depth above the root redirects
    // the whole store; the root itself being real is not enough.
    // (Quét ancestor (P0-A): symlink ở BẤT KỲ độ sâu nào phía trên root sẽ
    // chuyển hướng cả store — root thật chưa đủ.)
    check_cas_root_ancestors(root)
}

/// Current process UID (Unix) — for the CAS root ownership gate.
/// Uses the libc call under the SAME targeted allow as the COW clone path
/// in store.rs: one syscall, fixed return value, no memory involved.
/// (UID của process hiện tại (Unix) — cho cổng ownership gốc CAS. Dùng
/// lệnh libc dưới cùng allow cục bộ như đường COW clone trong store.rs:
/// một syscall, giá trị trả cố định, không chạm memory.)
#[cfg(unix)]
#[allow(unsafe_code)]
fn current_uid() -> u32 {
    // SAFETY: getuid() reads a fixed kernel value; no memory is touched.
    // (An toàn: getuid() đọc giá trị cố định của kernel; không chạm memory.)
    unsafe { libc::getuid() }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
