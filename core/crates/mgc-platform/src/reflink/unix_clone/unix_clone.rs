//! Unix `FICLONE` implementation for copy-on-write file cloning.
//! Implementation `FICLONE` trên Unix để clone file copy-on-write.

use crate::reflink::ReflinkError;
use std::io;
use std::path::Path;

/// Clone `source` to `target` via FICLONE (reflink) on btrfs/xfs/ext4(feature).
/// Dùng FICLONE để clone `source` sang `target` trên btrfs/xfs/ext4 (nếu bật hỗ trợ).
///
/// FICLONE copies data lazily (copy-on-write); target must not already exist.
/// FICLONE sao chép dữ liệu theo cơ chế copy-on-write; target không được tồn tại trước khi gọi.
/// Caller phải xóa target cũ trước; `O_EXCL` chặn việc ghi đè.
#[cfg(all(unix, not(target_os = "macos")))]
// SAFETY: every libc call below is a documented syscall wrapper taking only
// validated CStrings (interior NUL rejected via CString::new) and owned fds
// checked for success before use; all pointers/fds outlive their synchronous
// call, the kernel performs its own validation, and no memory is
// reinterpreted — failures surface as io::Error and map to NotSupported/Other.
// (AN TOÀN: mọi lệnh libc dưới đây là wrapper syscall có tài liệu, chỉ nhận
// CString đã validate và fd tự sở hữu đã kiểm tra thành công trước khi dùng;
// con trỏ/fd sống qua lời gọi đồng bộ, kernel tự validate, không diễn giải lại
// bộ nhớ — lỗi trả về io::Error rồi map sang NotSupported/Other.)
#[allow(unsafe_code)]
pub(in crate::reflink) fn reflink_clone(source: &Path, target: &Path) -> Result<(), ReflinkError> {
    use std::os::unix::ffi::OsStrExt;
    let src = std::ffi::CString::new(source.as_os_str().as_bytes()).map_err(|_| {
        ReflinkError::Other(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nul in source path",
        ))
    })?;
    let dst = std::ffi::CString::new(target.as_os_str().as_bytes()).map_err(|_| {
        ReflinkError::Other(io::Error::new(
            io::ErrorKind::InvalidInput,
            "nul in target path",
        ))
    })?;
    // SAFETY: open(2) reads only the NUL-terminated src CString (alive for the
    // sync call) plus O_RDONLY; the returned fd is checked (< 0) immediately.
    // (AN TOÀN: open(2) chỉ đọc CString src kết thúc NUL còn sống qua lời gọi
    // đồng bộ và cờ O_RDONLY; fd trả về được kiểm tra (< 0) ngay.)
    let src_fd = unsafe { libc::open(src.as_ptr(), libc::O_RDONLY) };
    if src_fd < 0 {
        return Err(ReflinkError::Other(io::Error::last_os_error()));
    }
    // SAFETY: open(2) with O_WRONLY|O_CREAT|O_EXCL reads only the
    // NUL-terminated dst CString (alive for the sync call); mode 0o644 applies
    // only because O_CREAT is set; the fd is checked (< 0) immediately.
    // (AN TOÀN: open(2) chỉ đọc CString dst kết thúc NUL còn sống qua lời gọi;
    // mode 0o644 chỉ có hiệu lực vì có O_CREAT; fd được kiểm tra ngay.)
    let dst_fd = unsafe {
        libc::open(
            dst.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
            0o644,
        )
    };
    if dst_fd < 0 {
        let err = io::Error::last_os_error();
        // SAFETY: src_fd came from a successful open above (>= 0), is owned by
        // this function, and is closed exactly once on this path.
        // (AN TOÀN: src_fd từ open thành công (>= 0), do hàm này sở hữu, đóng
        // đúng một lần trên nhánh này.)
        unsafe { libc::close(src_fd) };
        return Err(ReflinkError::Other(err));
    }
    // SAFETY: ioctl(fd, FICLONE, src_fd) passes the source fd by value (no
    // pointer dereference); both fds are owned open handles checked >= 0
    // above; the -1 return is handled via last_os_error below.
    // (AN TOÀN: ioctl FICLONE truyền fd nguồn theo giá trị, không deref con
    // trỏ; cả 2 fd là handle đang mở đã kiểm tra; lỗi -1 được xử lý bên dưới.)
    let rc = unsafe { libc::ioctl(dst_fd, libc::FICLONE as _, src_fd) };
    // SAFETY: both fds came from successful opens above, are owned by this
    // function, and are closed exactly once each here.
    // (AN TOÀN: cả 2 fd từ open thành công, do hàm này sở hữu, mỗi fd đóng
    // đúng một lần tại đây.)
    unsafe { libc::close(src_fd) };
    unsafe { libc::close(dst_fd) };
    if rc == 0 {
        Ok(())
    } else {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(code)
                if code == libc::EXDEV
                    || code == libc::EOPNOTSUPP
                    || code == libc::ENOTSUP
                    || code == libc::EINVAL
                    || code == libc::ENOSYS =>
            {
                Err(ReflinkError::NotSupported(err))
            }
            _ => Err(ReflinkError::Other(err)),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
