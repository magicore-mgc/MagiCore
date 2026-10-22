//! Detects Windows junctions, mount points, and other reparse attributes.
//! Nhận diện junction, mount point và reparse attribute khác trên Windows.

use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_REPARSE_POINT, GetFileAttributesW, INVALID_FILE_ATTRIBUTES,
};

/// Check `FILE_ATTRIBUTE_REPARSE_POINT` without following the path.
/// Kiểm tra `FILE_ATTRIBUTE_REPARSE_POINT` mà không follow path.
pub(in crate::project_lock::path_safety) fn windows_reparse_point(path: &Path) -> bool {
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: the nul-terminated buffer lives through this synchronous call; no memory is retained.
    // AN TOÀN: buffer NUL kết thúc sống hết lời gọi đồng bộ; hàm không giữ lại bộ nhớ.
    #[allow(unsafe_code)]
    let attrs = unsafe { GetFileAttributesW(wide.as_ptr()) };
    attrs != INVALID_FILE_ATTRIBUTES && (attrs & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
