//! Normalizes Flutter project paths and encodes file URIs.
//! Chuẩn hóa đường dẫn Flutter và mã hóa file URI.

use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;

use super::error::FlutterProjectUriError;

pub(super) fn flutter_project_root_uri_inner(
    project_root: &Path,
    current_dir: Option<&Path>,
) -> Result<String, FlutterProjectUriError> {
    let absolute_root = if project_root.is_absolute() {
        project_root.to_path_buf()
    } else {
        current_dir
            .ok_or(FlutterProjectUriError::MissingCurrentDirectory)?
            .join(project_root)
    };
    #[cfg(windows)]
    let absolute_root = expand_windows_short_path(&absolute_root)?;
    url::Url::from_directory_path(absolute_root)
        .map(|uri| uri.to_string())
        .map_err(|_| FlutterProjectUriError::EncodeProjectRoot)
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn expand_windows_short_path(path: &Path) -> Result<PathBuf, FlutterProjectUriError> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use windows_sys::Win32::Storage::FileSystem::GetLongPathNameW;

    // Expand DOS 8.3 aliases before Flutter compares the package root URI.
    // Bung tên DOS 8.3 trước khi Flutter so khớp URI root của package.
    let encoded = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // The input is NUL-terminated, and a null output with zero length is the documented size query.
    // Đầu vào có NUL kết thúc; output null với độ dài 0 là truy vấn kích thước theo API.
    let required = unsafe { GetLongPathNameW(encoded.as_ptr(), std::ptr::null_mut(), 0) };
    if required == 0 {
        return Err(FlutterProjectUriError::ExpandWindowsPath(
            std::io::Error::last_os_error(),
        ));
    }
    let mut buffer = vec![0_u16; required as usize];
    // The buffer has `required` writable UTF-16 slots from the size query above.
    // Buffer có `required` ô UTF-16 có thể ghi, lấy từ truy vấn kích thước ở trên.
    let written = unsafe { GetLongPathNameW(encoded.as_ptr(), buffer.as_mut_ptr(), required) };
    if written == 0 || written >= required {
        return Err(FlutterProjectUriError::ReadExpandedWindowsPath(
            std::io::Error::last_os_error(),
        ));
    }
    Ok(PathBuf::from(OsString::from_wide(
        &buffer[..written as usize],
    )))
}
