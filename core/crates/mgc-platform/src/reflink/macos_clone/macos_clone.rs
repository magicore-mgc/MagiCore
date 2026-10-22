//! macOS `clonefile(2)` implementation.
//! Implementation `clonefile(2)` dành cho macOS.

use crate::reflink::ReflinkError;
use std::io;
use std::path::Path;

/// Clone `source` to `target` as a copy-on-write reflink.
/// Clone `source` sang `target` bằng reflink copy-on-write.
///
/// Returns:
/// Trả về:
/// - `Ok(())` — clone created (target must not pre-exist).
/// - `Ok(())` — tạo clone thành công (target chưa được tồn tại).
/// - `Err(NotSupported)` — platform/fs cannot reflink (caller falls back).
/// - `Err(NotSupported)` — OS/filesystem không hỗ trợ (caller chạy fallback).
/// - `Err(other)` — real failure that should abort the operation.
/// - `Err(other)` — lỗi thật cần làm thao tác dừng lại.
#[cfg(target_os = "macos")]
// SAFETY: clonefile(2) is a documented syscall taking only the two
// NUL-terminated CStrings built above (interior NUL rejected via
// CString::new) plus flags 0, the only documented value; both pointers
// outlive the synchronous call, the kernel performs its own path validation,
// and no memory is reinterpreted.
// (AN TOÀN: clonefile(2) là syscall có tài liệu, chỉ nhận 2 CString kết thúc
// NUL đã loại NUL giữa chuỗi qua CString::new và cờ 0 duy nhất có tài liệu;
// con trỏ sống qua lời gọi đồng bộ, kernel tự validate path, không diễn giải
// lại bộ nhớ.)
#[allow(unsafe_code)]
pub(in crate::reflink) fn reflink_clone(source: &Path, target: &Path) -> Result<(), ReflinkError> {
    use std::ffi::CString;
    let src = CString::new(source.as_os_str().as_encoded_bytes()).map_err(|_| {
        ReflinkError::Other(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nul in source path",
        ))
    })?;
    let dst = CString::new(target.as_os_str().as_encoded_bytes()).map_err(|_| {
        ReflinkError::Other(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nul in target path",
        ))
    })?;
    // clonefile(src, dst, 0): fails EXDEV across volumes, ENOTSUP on unsupported fs.
    // clonefile(src, dst, 0): lỗi EXDEV giữa volume, ENOTSUP nếu filesystem không hỗ trợ.
    // SAFETY: per the fn-level note — validated CStrings, sync call, rc checked.
    // (AN TOÀN: theo ghi chú cấp hàm — CString đã validate, gọi đồng bộ, đã kiểm tra rc.)
    let rc = unsafe { libc::clonefile(src.as_ptr(), dst.as_ptr(), 0) };
    if rc == 0 {
        Ok(())
    } else {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(libc::EXDEV)
            | Some(libc::ENOTSUP)
            | Some(libc::EOPNOTSUPP)
            | Some(libc::ENOSYS) => Err(ReflinkError::NotSupported(err)),
            _ => Err(ReflinkError::Other(err)),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
