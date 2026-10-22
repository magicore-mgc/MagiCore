//! Armed temp-file guard for crash-safe cache publication.
//! Guard file tạm đang kích hoạt cho publish cache an toàn khi crash.

/// RAII cleanup for compiled-cache temp files (audit vòng-3 P0-5).
/// (Dọn temp compiled-cache bằng RAII — P0-5 audit vòng-3.)
use std::fs;
pub(in crate::cas::compiled_cache) struct TmpGuard<'a> {
    path: &'a std::path::Path,
    armed: bool,
}

impl<'a> TmpGuard<'a> {
    pub(in crate::cas::compiled_cache) fn new(path: &'a std::path::Path) -> Self {
        Self { path, armed: true }
    }

    pub(in crate::cas::compiled_cache) fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TmpGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            // Best-effort removal of our own unique temp.
            // (Best-effort xóa temp duy nhất của mình.)
            let _ = fs::remove_file(self.path);
        }
    }
}
