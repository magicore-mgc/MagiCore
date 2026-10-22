//! Opens one regular file without following its final symlink/reparse point.
//! Mở file thường mà không đi theo symlink/reparse ở thành phần cuối.

#[cfg(any(unix, windows))]
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::path::Path;

use super::error::OpenRegularFileError;

/// Open a regular file without following its final path component.
/// Mở file thường, không đi theo link ở thành phần cuối của path.
///
/// Unix uses `O_NOFOLLOW | O_NONBLOCK`; Windows opens the reparse object and
/// rejects its handle metadata. Parent components must be confined by caller.
/// Unix dùng `O_NOFOLLOW | O_NONBLOCK`; Windows mở reparse object rồi từ chối
/// metadata của handle. Caller chịu trách nhiệm giới hạn các thành phần cha.
pub fn open_regular_file_no_follow(path: &Path) -> Result<File, OpenRegularFileError> {
    let path_metadata = fs::symlink_metadata(path)?;
    if path_metadata.file_type().is_symlink() {
        return Err(OpenRegularFileError::Symlink);
    }
    if !path_metadata.is_file() {
        return Err(OpenRegularFileError::NotRegular);
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = path;
        return Err(OpenRegularFileError::UnsupportedPlatform);
    }

    #[cfg(any(unix, windows))]
    {
        let mut options = OpenOptions::new();
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

        let file = options.open(path).map_err(classify_open_error)?;
        let opened_metadata = file.metadata()?;
        if !opened_metadata.is_file() {
            return Err(OpenRegularFileError::OpenedNotRegular);
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
            if opened_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                return Err(OpenRegularFileError::ReparsePoint);
            }
        }
        Ok(file)
    }
}

fn classify_open_error(source: std::io::Error) -> OpenRegularFileError {
    #[cfg(unix)]
    if source.raw_os_error() == Some(libc::ELOOP) {
        return OpenRegularFileError::Symlink;
    }
    OpenRegularFileError::Io(source)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
