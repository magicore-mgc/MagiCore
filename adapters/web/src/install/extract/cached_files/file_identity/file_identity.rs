//! Host-specific opened-file identity and symlink/reparse-safe opening.
//! Định danh handle và mở file chống symlink/reparse theo OS.
use super::super::types::CachedFileIdentity;
use std::fs::{File, Metadata, OpenOptions};
use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;

#[allow(unsafe_code)]
pub(crate) fn cached_file_identity(
    file: &File,
    _metadata: &Metadata,
) -> Option<CachedFileIdentity> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let _ = file;
        Some((_metadata.dev(), _metadata.ino()))
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{
            BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
        };

        let mut information = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
        // SAFETY: `file` owns a live handle and `information` is writable for
        // the full structure; Windows initializes it when the call succeeds.
        // (AN TOÀN: `file` sở hữu handle còn sống và `information` ghi được
        // toàn bộ struct; Windows khởi tạo struct khi lời gọi thành công.)
        let succeeded = unsafe {
            GetFileInformationByHandle(
                file.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE,
                information.as_mut_ptr(),
            )
        };
        if succeeded == 0 {
            return None;
        }
        // SAFETY: Windows returned success, so it initialized every field.
        // (AN TOÀN: Windows trả thành công nên mọi field đã được khởi tạo.)
        let information = unsafe { information.assume_init() };
        let file_index =
            (u64::from(information.nFileIndexHigh) << 32) | u64::from(information.nFileIndexLow);
        Some((information.dwVolumeSerialNumber, file_index))
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (file, _metadata);
        Some(())
    }
}

/// Open one cache file without accepting a final symlink/reparse point.
/// Mở file cache và từ chối symlink/reparse point ở thành phần cuối.
pub(in crate::install::extract::cached_files) fn open_cached_file_no_follow(
    package_root: &Path,
    relative_path: &Path,
) -> std::io::Result<Option<File>> {
    use std::path::Component;

    if relative_path.as_os_str().is_empty()
        || !relative_path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Ok(None);
    }

    #[cfg(unix)]
    let file = {
        use rustix::fs::{Mode, OFlags, openat};
        use rustix::io::Errno;
        use std::os::fd::AsFd;
        use std::os::unix::fs::OpenOptionsExt;

        fn missing_or_link(error: Errno) -> bool {
            matches!(error, Errno::NOENT | Errno::LOOP | Errno::NOTDIR)
        }

        let mut directory = match OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(package_root)
        {
            Ok(directory) => directory,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) if error.raw_os_error() == Some(libc::ELOOP) => return Ok(None),
            Err(error) => return Err(error),
        };
        let components = relative_path.components().collect::<Vec<_>>();
        for component in &components[..components.len() - 1] {
            let Component::Normal(name) = component else {
                return Ok(None);
            };
            // Resolve each directory from the already-open parent handle; a path-level
            // check alone could follow a replaced ancestor symlink.
            // Mở từng thư mục tương đối với handle cha; chỉ kiểm tra chuỗi path có thể đi theo symlink tổ tiên đã bị thay.
            let opened = openat(
                directory.as_fd(),
                *name,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            );
            directory = match opened {
                Ok(opened) => File::from(opened),
                Err(error) if missing_or_link(error) => return Ok(None),
                Err(error) => return Err(error.into()),
            };
        }

        let Component::Normal(name) = &components[components.len() - 1] else {
            return Ok(None);
        };
        match openat(
            directory.as_fd(),
            *name,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
            Mode::empty(),
        ) {
            Ok(opened) => File::from(opened),
            Err(error) if missing_or_link(error) => return Ok(None),
            Err(error) => return Err(error.into()),
        }
    };
    #[cfg(windows)]
    let file = {
        let target = package_root.join(relative_path);
        if !windows_cached_path_is_safe(package_root, relative_path)? {
            return Ok(None);
        }
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
        let root_handle = match OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_BACKUP_SEMANTICS)
            .open(package_root)
        {
            Ok(root) => root,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if root_handle.metadata()?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Ok(None);
        }
        let root_final_path = windows_final_path(&root_handle)?;
        let file = match OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(&target)
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        if file.metadata()?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Ok(None);
        }
        if !windows_opened_path_is_within(&root_final_path, &windows_final_path(&file)?) {
            return Ok(None);
        }
        if !windows_cached_path_is_safe(package_root, relative_path)? {
            return Ok(None);
        }
        file
    };
    #[cfg(not(any(unix, windows)))]
    let file = match File::open(package_root.join(relative_path)) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };

    if !file.metadata()?.is_file() {
        return Ok(None);
    }
    Ok(Some(file))
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn windows_final_path(file: &File) -> std::io::Result<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_NAME_NORMALIZED, GetFinalPathNameByHandleW, VOLUME_NAME_DOS,
    };

    let handle = file.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;
    // SAFETY: `file` owns a valid open Windows handle; a null output buffer with
    // length zero is the documented size query for this API.
    // (AN TOÀN: `file` sở hữu handle Windows đang mở; buffer null với độ dài 0
    // là cách API quy định để hỏi kích thước.)
    let required = unsafe {
        GetFinalPathNameByHandleW(
            handle,
            std::ptr::null_mut(),
            0,
            FILE_NAME_NORMALIZED | VOLUME_NAME_DOS,
        )
    };
    if required == 0 {
        return Err(std::io::Error::last_os_error());
    }
    let mut buffer = vec![0u16; required as usize];
    // SAFETY: `buffer` is writable for `required` UTF-16 code units and the
    // handle remains owned by `file` for the duration of the call.
    // (AN TOÀN: `buffer` ghi được `required` đơn vị UTF-16 và `file` vẫn giữ
    // handle hợp lệ trong suốt lời gọi.)
    let written = unsafe {
        GetFinalPathNameByHandleW(
            handle,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            FILE_NAME_NORMALIZED | VOLUME_NAME_DOS,
        )
    };
    if written == 0 || written >= buffer.len() as u32 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(PathBuf::from(std::ffi::OsString::from_wide(
        &buffer[..written as usize],
    )))
}

#[cfg(windows)]
pub(crate) fn windows_opened_path_is_within(root: &Path, target: &Path) -> bool {
    use std::os::windows::ffi::OsStrExt;

    let root = root.as_os_str().encode_wide().collect::<Vec<_>>();
    let target = target.as_os_str().encode_wide().collect::<Vec<_>>();
    if target.len() <= root.len() || !target.starts_with(&root) {
        return false;
    }
    root.last() == Some(&(b'\\' as u16)) || target.get(root.len()) == Some(&(b'\\' as u16))
}

#[cfg(windows)]
fn windows_cached_path_is_safe(package_root: &Path, relative_path: &Path) -> std::io::Result<bool> {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    let root_metadata = std::fs::symlink_metadata(package_root)?;
    if root_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        return Ok(false);
    }
    let canonical_root = std::fs::canonicalize(package_root)?;
    let mut current = package_root.to_path_buf();
    for component in relative_path.components() {
        current.push(component.as_os_str());
        let metadata = std::fs::symlink_metadata(&current)?;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Ok(false);
        }
    }
    Ok(std::fs::canonicalize(current)?.starts_with(canonical_root))
}
