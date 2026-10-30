//! Reject ZIP paths that can escape extraction root — chặn path ZIP thoát khỏi thư mục đích.

use mgc_types::{MgError, MgResult};
use std::path::{Component, Path, PathBuf};

/// Reject any zip entry path that could escape the destination root.
/// Từ chối mọi path entry zip có thể thoát khỏi gốc đích.
pub(in crate::protocols::zip_reader) fn sanitize_zip_path(path: &str) -> MgResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => {
                reject_windows_unsafe_component(part, path)?;
                clean.push(part);
            }
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(MgError::Other(format!("unsafe zip entry path: {path}")));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(MgError::Other(format!("empty zip entry path: {path}")));
    }
    Ok(clean)
}

// Reject Windows-unsafe components: NTFS ADS (`file:ads`), trailing dot/space,
// and reserved device names. On Windows these resolve to a different file than
// the archive names. — Từ chối component không an toàn trên Windows: NTFS ADS
// (`file:ads`), dấu chấm/khoảng trắng cuối, tên thiết bị dành riêng; trên Windows
// chúng trỏ tới file khác tên archive khai.
fn reject_windows_unsafe_component(part: &std::ffi::OsStr, path: &str) -> MgResult<()> {
    let Some(text) = part.to_str() else {
        return Err(MgError::Other(format!("non-UTF-8 zip entry path: {path}")));
    };
    if text.contains(':') || text.ends_with('.') || text.ends_with(' ') {
        return Err(MgError::Other(format!("unsafe zip entry path: {path}")));
    }
    let stem = text.split('.').next().unwrap_or(text).trim();
    let upper = stem.to_ascii_uppercase();
    let reserved = matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (upper.len() == 4
            && (upper.starts_with("COM") || upper.starts_with("LPT"))
            && upper.as_bytes()[3].is_ascii_digit()
            && upper.as_bytes()[3] != b'0');
    if reserved {
        return Err(MgError::Other(format!("unsafe zip entry path: {path}")));
    }
    Ok(())
}
