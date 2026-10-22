//! Obtains a stable Unix file fingerprint or disables memoization elsewhere.
//! Lấy fingerprint file Unix ổn định hoặc tắt memo trên platform khác.

#[cfg(unix)]
use crate::cas::store::types::UnixFileIdentity;
use std::fs::Metadata;

/// Fingerprint opened-handle metadata for memo purposes (Unix).
/// Lấy fingerprint metadata của handle đã mở để dùng trong memo.
#[cfg(unix)]
pub(in crate::cas::store::verify) fn file_identity(meta: &Metadata) -> Option<UnixFileIdentity> {
    use std::os::unix::fs::MetadataExt;
    let mtime_nanos = meta.mtime() as u64 * 1_000_000_000 + meta.mtime_nsec() as u64;
    // ctime (status-change time): kernel-maintained, user-restore impossible
    // — the anti-Bypass-A core of the fingerprint.
    // (ctime (status-change time): kernel giữ, không thể phục hồi — lõi
    // chống Bypass A của fingerprint.)
    let ctime_nanos = meta.ctime() as u64 * 1_000_000_000 + meta.ctime_nsec() as u64;
    Some(UnixFileIdentity {
        dev: meta.dev(),
        ino: meta.ino(),
        size: meta.size(),
        mtime_nanos,
        ctime_nanos,
    })
}

/// Windows/other: no cheap stable file identity (NTFS file IDs rotate
/// across reboot; BirthId APIs are niche) — never memoize, always rehash
/// (exactly what the audit prescribes for missing stable identity).
/// (Windows/khác: không có file identity ổn định rẻ (NTFS file ID xoay
/// vòng sau reboot; API BirthId quá niche) — không memo hóa, luôn rehash
/// (đúng như audit chỉ dẫn khi thiếu stable identity).)
#[cfg(not(unix))]
pub(in crate::cas::store::verify) fn file_identity(
    _metadata: &Metadata,
) -> Option<std::convert::Infallible> {
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
