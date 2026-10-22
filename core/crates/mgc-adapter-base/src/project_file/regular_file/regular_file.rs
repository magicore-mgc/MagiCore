//! Open and read a regular file without following its final link.
//! Mở và đọc file thường mà không theo symlink tại thành phần cuối.

use super::super::error::{error, error_with_source};
use mgc_types::MgError;
use std::path::Path;

pub(in crate::project_file) fn reject_non_regular(
    path: &Path,
) -> Result<std::fs::Metadata, MgError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|source| {
        error_with_source(source, |e| {
            format!("inspect manifest {}: {e}", path.display())
        })
    })?;
    if !metadata.file_type().is_file() {
        return Err(error(format!(
            "refusing non-regular or symlink manifest: {}",
            path.display()
        )));
    }
    Ok(metadata)
}

/// Open the final path component without following a symlink/reparse point,
/// verify the opened handle is a regular file, and read from that handle.
/// Mở thành phần cuối không theo symlink/reparse, xác nhận handle là file thường rồi đọc.
pub(in crate::project_file) fn read_regular_text(
    path: &Path,
    display_name: &str,
) -> Result<String, MgError> {
    use std::io::Read;

    // Refuse FIFOs and other special files before opening; O_NONBLOCK below
    // closes the replacement race between this check and the actual open.
    // Từ chối FIFO/file đặc biệt trước khi mở; O_NONBLOCK chặn race thay path.
    reject_non_regular(path)?;

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let mut file = options.open(path).map_err(|source| {
        error_with_source(source, |e| {
            format!("open {display_name} without following links: {e}")
        })
    })?;
    let metadata = file.metadata().map_err(|source| {
        error_with_source(source, |e| format!("inspect opened {display_name}: {e}"))
    })?;
    if !metadata.is_file() {
        return Err(error(format!("{display_name} is not a regular file")));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(error(format!(
                "refusing reparse-point manifest {display_name}"
            )));
        }
    }
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|source| error_with_source(source, |e| format!("read {display_name}: {e}")))?;
    Ok(content)
}

#[cfg(test)]
#[path = "test/regular_file.rs"]
mod tests;
