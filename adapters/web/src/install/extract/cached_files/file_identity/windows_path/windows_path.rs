//! Windows path-safety helpers: final handle path, containment and reparse-point checks.
//! Helper an toàn path Windows: final path theo handle, kiểm chứa và reparse point.

use std::fs::File;
use std::path::{Path, PathBuf};

#[cfg(windows)]
#[allow(unsafe_code)]
pub(crate) fn windows_final_path(file: &File) -> std::io::Result<PathBuf> {
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
pub(crate) fn windows_cached_path_is_safe(
    package_root: &Path,
    relative_path: &Path,
) -> std::io::Result<bool> {
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
