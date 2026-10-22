//! Read one bounded regular project file without following its final link.
//! Đọc một file project thường có giới hạn, không theo link cuối.
use std::io::Read;
use std::path::Path;

use super::super::error::{FileAccessError, Result};

const MAX_PROJECT_TEXT_BYTES: u64 = 10 * 1024 * 1024;

/// Stable public reader; keep the anyhow return boundary for existing callers.
/// Reader public ổn định; giữ biên anyhow cho caller hiện tại.
pub fn read_regular_project_text(path: &Path, label: &str) -> anyhow::Result<Option<String>> {
    read_regular_project_text_typed(path, label).map_err(FileAccessError::into_anyhow)
}

/// Read with path checks, no-follow open flags and an opened-handle check.
/// Đọc sau khi kiểm tra path, mở no-follow và kiểm tra metadata của handle.
pub(in crate::project::project::file_access) fn read_regular_project_text_typed(
    path: &Path,
    label: &str,
) -> Result<Option<String>> {
    let path_metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if path_metadata.file_type().is_symlink() {
        return Err(FileAccessError::Symlink {
            label: label.to_owned(),
            path: path.to_path_buf(),
        });
    }
    if !path_metadata.is_file() {
        return Err(FileAccessError::NotRegular {
            label: label.to_owned(),
            path: path.to_path_buf(),
        });
    }

    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options
        .open(path)
        .map_err(|source| FileAccessError::OpenNoFollow {
            label: label.to_owned(),
            path: path.to_path_buf(),
            source,
        })?;
    let opened_metadata = file.metadata()?;
    if !opened_metadata.is_file() {
        return Err(FileAccessError::OpenedNotRegular {
            label: label.to_owned(),
            path: path.to_path_buf(),
        });
    }
    if opened_metadata.len() > MAX_PROJECT_TEXT_BYTES {
        return Err(FileAccessError::TooLarge {
            label: label.to_owned(),
            path: path.to_path_buf(),
            limit: MAX_PROJECT_TEXT_BYTES,
        });
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if opened_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(FileAccessError::ReparsePoint {
                label: label.to_owned(),
                path: path.to_path_buf(),
            });
        }
    }

    let mut content = String::new();
    file.take(MAX_PROJECT_TEXT_BYTES + 1)
        .read_to_string(&mut content)?;
    if content.len() as u64 > MAX_PROJECT_TEXT_BYTES {
        return Err(FileAccessError::TooLarge {
            label: label.to_owned(),
            path: path.to_path_buf(),
            limit: MAX_PROJECT_TEXT_BYTES,
        });
    }
    Ok(Some(content))
}
