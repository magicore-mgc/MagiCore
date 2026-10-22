//! Bounds in-process duplicate publication without weakening file verification.
//! Giới hạn publish trùng trong process, giữ nguyên kiểm tra file bị sửa.

use std::sync::OnceLock;

use crate::cas::integrity::IntegrityHash;

const IMPORT_LOCK_STRIPES: usize = 256;
static IMPORT_LOCKS: OnceLock<[parking_lot::Mutex<()>; IMPORT_LOCK_STRIPES]> = OnceLock::new();

/// Share bounded locks across independent handles; distinct digests remain concurrent.
/// Dùng khóa có giới hạn giữa mọi handle; các digest khác vẫn chạy song song.
pub(in crate::cas::store::import) fn lock_digest(
    hash: &IntegrityHash,
) -> parking_lot::MutexGuard<'static, ()> {
    let index = hash.as_hex().bytes().fold(0usize, |index, byte| {
        (index * 31 + usize::from(byte)) % IMPORT_LOCK_STRIPES
    });
    let locks = IMPORT_LOCKS.get_or_init(|| std::array::from_fn(|_| parking_lot::Mutex::new(())));
    locks[index].lock()
}
