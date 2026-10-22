//! Reject ZIP paths that can escape extraction root — chặn path ZIP thoát khỏi thư mục đích.

use mgc_types::{MgError, MgResult};
use std::path::{Component, Path, PathBuf};

/// Reject any zip entry path that could escape the destination root.
/// Từ chối mọi path entry zip có thể thoát khỏi gốc đích.
pub(in crate::protocols::zip_reader) fn sanitize_zip_path(path: &str) -> MgResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => clean.push(part),
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
