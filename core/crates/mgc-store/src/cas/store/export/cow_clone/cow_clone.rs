//! Clones from an already-open source descriptor when the filesystem allows it.
//! Clone từ descriptor nguồn đã mở nếu filesystem hỗ trợ.

#[cfg(target_os = "linux")]
use std::fs;
use std::fs::File;
use std::path::Path;

/// Open a regular source without following its final component for path probes.
/// Mở file nguồn thường và không theo symlink cuối cho các probe dùng path.
pub(in crate::cas::store) fn try_cow_clone(src: &Path, tmp: &Path) -> Option<()> {
    let source = mgc_platform::open_regular_file_no_follow(src).ok()?;
    try_cow_clone_from_file(&source, tmp)
}

/// Try platform copy-on-write cloning; the caller owns plain-copy fallback.
/// Thử COW theo platform; caller sở hữu fallback copy thường.
///
/// macOS uses `fclonefileat` with the source fd; Linux uses `FICLONE` with
/// the same fd. Windows and unsupported targets report no fast path.
/// macOS dùng `fclonefileat` với source fd; Linux dùng `FICLONE` cùng fd.
/// Windows và target chưa hỗ trợ báo không có fast path.
///
/// Failed Linux clone attempts remove only the empty destination file created
/// here. macOS `fclonefileat` creates the destination atomically or not at all.
/// Clone Linux thất bại thì chỉ xóa file đích rỗng do hàm này tạo. macOS
/// `fclonefileat` tạo đích nguyên tử hoặc không tạo.
///
/// SAFETY: `fclonefileat` receives a live fd owned by `source`, a valid
/// NUL-terminated destination path, and flags zero. `ioctl(FICLONE)` receives
/// live file descriptors; the kernel reads the source-fd integer by value.
/// (An toàn: `fclonefileat` nhận fd còn sống từ `source`, path đích NUL hợp
/// lệ và flags zero. `ioctl(FICLONE)` nhận descriptor file còn sống; kernel
/// đọc số source-fd theo giá trị.)
#[allow(unsafe_code)]
#[allow(unused_variables)]
pub(in crate::cas::store::export) fn try_cow_clone_from_file(
    source: &File,
    tmp: &Path,
) -> Option<()> {
    #[cfg(target_os = "macos")]
    {
        use std::ffi::CString;
        use std::os::fd::AsRawFd;
        use std::os::unix::ffi::OsStrExt;

        let Ok(tmp_c) = CString::new(tmp.as_os_str().as_bytes()) else {
            return None;
        };
        let result = unsafe {
            // SAFETY: source owns the valid fd; tmp_c remains alive for the call.
            // (An toàn: source sở hữu fd hợp lệ; tmp_c còn sống trong suốt lời gọi.)
            libc::fclonefileat(source.as_raw_fd(), libc::AT_FDCWD, tmp_c.as_ptr(), 0)
        };
        (result == 0).then_some(())
    }

    #[cfg(target_os = "linux")]
    {
        use std::os::fd::AsRawFd;
        const FICLONE: libc::c_ulong = 0x40049409;

        let tmp_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(tmp)
            .ok()?;
        let result = unsafe {
            // SAFETY: both descriptors stay live during ioctl; FICLONE reads
            // the source descriptor number as an integer argument.
            // (An toàn: hai descriptor còn sống trong ioctl; FICLONE đọc số fd
            // nguồn như một đối số số nguyên.)
            libc::ioctl(tmp_file.as_raw_fd(), FICLONE, source.as_raw_fd())
        };
        if result == 0 {
            Some(())
        } else {
            drop(tmp_file);
            let _ = fs::remove_file(tmp);
            None
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        None
    }
}
