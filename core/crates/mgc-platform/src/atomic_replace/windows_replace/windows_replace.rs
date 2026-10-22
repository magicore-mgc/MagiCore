//! Replaces a Windows file with caller-selected write-through and extended paths.
//! Thay file Windows với write-through do caller chọn và đường dẫn mở rộng.

use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use windows_sys::Win32::Storage::FileSystem::{
    MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
};

use super::super::atomic_replace::replacement_error;

fn wide(path: &Path) -> Vec<u16> {
    path.as_os_str().encode_wide().chain(Some(0)).collect()
}

fn extended_path(path: &Path) -> std::io::Result<PathBuf> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "atomic path has no file name",
        )
    })?;
    Ok(std::fs::canonicalize(parent)?.join(name))
}

#[allow(unsafe_code)]
pub(super) fn windows_replace(
    staging: &Path,
    destination: &Path,
    write_through: bool,
) -> Result<(), super::super::AtomicReplaceError> {
    let from_path =
        extended_path(staging).map_err(|source| replacement_error(staging, destination, source))?;
    let to_path = extended_path(destination)
        .map_err(|source| replacement_error(staging, destination, source))?;
    let from = wide(&from_path);
    let to = wide(&to_path);
    // SAFETY: NUL-terminated buffers live through the synchronous call; Windows does not retain them.
    // AN TOÀN: buffer có NUL sống hết lời gọi đồng bộ; Windows không giữ các buffer này.
    let mut flags = MOVEFILE_REPLACE_EXISTING;
    if write_through {
        flags |= MOVEFILE_WRITE_THROUGH;
    }
    let result = unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), flags) };
    if result == 0 {
        return Err(replacement_error(
            staging,
            destination,
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}

#[cfg(all(test, windows))]
#[path = "test/mod.rs"]
mod test;
