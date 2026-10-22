//! Stage, sync, and atomically publish one extracted regular file.
//! Ghi staging, đồng bộ và publish nguyên tử một file giải nén.

use super::super::{ArchiveError, ArchiveResult};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static EXTRACT_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Write extracted bytes via a unique same-directory temp and rename so the
/// final path is not opened through a swapped symlink. Windows uses an
/// atomic replace primitive rather than remove-then-rename.
/// Ghi byte qua temp duy nhất cùng thư mục rồi rename để không mở file đích
/// qua symlink bị thay giữa chừng.
pub(in crate::protocols::archive) fn write_extracted_file(
    path: &Path,
    contents: &[u8],
) -> ArchiveResult<()> {
    use std::io::Write;

    let parent = path.parent().ok_or_else(|| {
        ArchiveError::Integrity("archive target has no parent directory".to_string())
    })?;
    let stem = path
        .file_name()
        .ok_or_else(|| ArchiveError::Integrity("archive target has no filename".to_string()))?;
    let mut temp_path = None;
    let mut temp_file = None;
    for _ in 0..32 {
        let sequence = EXTRACT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".{}.mgc-extract-{}-{sequence}.tmp",
            stem.to_string_lossy(),
            std::process::id()
        ));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            Ok(file) => {
                temp_path = Some(candidate);
                temp_file = Some(file);
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }
    let temp_path = temp_path.ok_or_else(|| {
        ArchiveError::Other("could not allocate unique archive extraction temp".to_string())
    })?;
    let result = (|| {
        let mut file = temp_file.expect("temp path and file are created together");
        file.write_all(contents)?;
        file.sync_all()?;
        drop(file);
        atomic_publish_extracted_file(&temp_path, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp_path);
    }
    result
}

#[cfg(not(windows))]
fn atomic_publish_extracted_file(temp_path: &Path, path: &Path) -> ArchiveResult<()> {
    std::fs::rename(temp_path, path)?;
    Ok(())
}

#[cfg(windows)]
#[allow(unsafe_code)]
fn atomic_publish_extracted_file(temp_path: &Path, path: &Path) -> ArchiveResult<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let from: Vec<u16> = temp_path.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: both nul-terminated path buffers remain alive for the synchronous call.
    // (AN TOÀN: hai buffer đường dẫn kết thúc nul còn sống trong lời gọi đồng bộ.)
    let result = unsafe {
        MoveFileExW(
            from.as_ptr(),
            to.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}
