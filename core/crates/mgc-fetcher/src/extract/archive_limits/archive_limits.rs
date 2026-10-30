//! Named limits for untrusted tar expansion.
//! Giới hạn có tên cho việc bung tar không tin cậy.

pub(in crate::extract) const MAX_ARCHIVE_ENTRIES: usize = 16_384;
pub(in crate::extract) const MAX_ENTRY_BYTES: u64 = 256 * 1024 * 1024;
pub(in crate::extract) const MAX_TOTAL_FILE_BYTES: u64 = 512 * 1024 * 1024;

// SEC-3: entries that execute code at interpreter/tool start rather than when
// the package is imported. Detection mirrors the resolver's shared detector;
// the two crates cannot depend on each other (mgc-resolver depends on
// mgc-store, and mgc-fetcher is an L1 download/extract helper), so the list is
// duplicated here with the same normalization rules and the same test matrix.
// (SEC-3: entry chạy code khi interpreter/tool khởi động chứ không phải khi
// package được import. Detector phản chiếu detector dùng chung của resolver;
// hai crate không phụ thuộc nhau (mgc-resolver phụ thuộc mgc-store, mgc-fetcher
// là helper tải/giải nén L1), nên danh sách được lặp tại đây với cùng quy tắc
// chuẩn hóa và cùng ma trận test.)
pub(in crate::extract) const PYTHON_SITE_HOOK_MODULES: &[&str] =
    &["sitecustomize.py", "usercustomize.py"];
pub(in crate::extract) const NUGET_INSTALL_SCRIPT_BASENAMES: &[&str] = &["install.ps1", "init.ps1"];
pub(in crate::extract) const FORBIDDEN_MODE_BITS: u32 = 0o6000;

/// File-type bits of a POSIX mode word (`S_IFMT`).
/// Bit loại file trong mode POSIX (`S_IFMT`).
pub(in crate::extract) const FILE_TYPE_BITS: u32 = 0o170000;

/// Symbolic-link file type (`S_IFLNK`).
/// Loại file symlink (`S_IFLNK`).
pub(in crate::extract) const SYMLINK_FILE_TYPE: u32 = 0o120000;

/// Whether a POSIX mode word declares a symbolic link.
/// Mode POSIX có khai báo symlink không.
///
/// Zip entries carry `S_IFLNK` in their external attributes and no tar-style
/// `is_symlink` flag, so a mode check that only masks the privilege bits lets a
/// link entry through. CVE-2026-5223 is the same class in Cargo: a symlink in
/// one crate archive overwrote another crate's cache entry.
/// (Entry zip mang `S_IFLNK` trong external attributes và không có cờ
/// `is_symlink` như tar, nên phép kiểm mode chỉ mask bit đặc quyền sẽ để entry
/// symlink lọt qua. CVE-2026-5223 là cùng lớp lỗi ở Cargo: symlink trong một
/// archive crate ghi đè cache của crate khác.)
pub(in crate::extract) fn mode_declares_symlink(mode: u32) -> bool {
    mode & FILE_TYPE_BITS == SYMLINK_FILE_TYPE
}
