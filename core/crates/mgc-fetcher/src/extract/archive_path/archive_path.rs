//! Normalize archive entry paths and reject traversal.
//! Chuẩn hóa path entry archive và từ chối traversal.

use super::super::{ExtractionError, ExtractionResult};
use std::path::{Component, Path, PathBuf};

pub(in crate::extract) fn sanitize_archive_path(path: &Path) -> ExtractionResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                reject_windows_unsafe_component(part, path)?;
                clean.push(part);
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(ExtractionError::EmptyArchivePath);
    }
    Ok(clean)
}

// Reject Windows-unsafe entry components: NTFS alternate data streams (`file:ads`),
// trailing dot/space (silently stripped by Win32, alias to another name), and
// reserved device names (`CON`, `NUL`, `COM1`...). These map to a different file
// than the archive names on Windows, which is an extraction-escape vector.
// Từ chối component không an toàn trên Windows: NTFS ADS (`file:ads`), dấu chấm/khoảng
// trắng cuối (Win32 tự cắt, alias sang tên khác), và tên thiết bị dành riêng
// (`CON`, `NUL`, `COM1`...). Trên Windows chúng trỏ tới file khác tên archive khai —
// là vector thoát khỏi thư mục giải nén.
fn reject_windows_unsafe_component(part: &std::ffi::OsStr, path: &Path) -> ExtractionResult<()> {
    let Some(text) = part.to_str() else {
        return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
    };
    if text.contains(':') {
        return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
    }
    if text.ends_with('.') || text.ends_with(' ') {
        return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
    }
    let stem = text.split('.').next().unwrap_or(text).trim();
    let upper = stem.to_ascii_uppercase();
    let reserved = matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (upper.len() == 4
            && (upper.starts_with("COM") || upper.starts_with("LPT"))
            && upper.as_bytes()[3].is_ascii_digit()
            && upper.as_bytes()[3] != b'0');
    if reserved {
        return Err(ExtractionError::UnsafeArchivePath(path.to_path_buf()));
    }
    Ok(())
}
