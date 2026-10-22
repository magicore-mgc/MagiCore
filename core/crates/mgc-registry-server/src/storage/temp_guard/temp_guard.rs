//! Owns RAII cleanup for unique temporary files used by atomic blob writes.
//! Sở hữu RAII cleanup cho file tạm duy nhất dùng trong ghi blob nguyên tử.

use super::super::prelude::*;

/// RAII cleanup for atomic-write temp files — removes the temp on drop
/// unless disarmed after a successful rename (P0-5).
/// Dọn temp của ghi nguyên tử bằng RAII — xóa temp khi drop trừ khi disarm
/// sau khi rename thành công (P0-5).
pub(in crate::storage) struct TempGuard<'a> {
    path: &'a Path,
    armed: bool,
}

impl<'a> TempGuard<'a> {
    pub(in crate::storage) fn new(path: &'a Path) -> Self {
        Self { path, armed: true }
    }

    /// Disarm after the temp has been renamed away (nothing to clean).
    /// Disarm sau khi temp đã được rename đi (không còn gì để dọn).
    pub(in crate::storage) fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TempGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            // Best-effort: sync removal of our own unique temp.
            // Best-effort: xóa sync temp duy nhất của mình.
            let _ = std::fs::remove_file(self.path);
        }
    }
}

/// Unique temp path inside `parent` for atomic local writes.
/// Đường dẫn temp duy nhất trong `parent` cho ghi local nguyên tử.
pub(in crate::storage) fn parent_temp_path(parent: &Path) -> PathBuf {
    parent.join(format!(
        ".put-{}-{}",
        uuid::Uuid::new_v4(),
        std::process::id()
    ))
}
