//! Reject unsafe archive paths and normalize safe relative paths.
//! Từ chối path archive không an toàn và chuẩn hóa path tương đối hợp lệ.

use std::path::{Path, PathBuf};

use mgc_types::{MgError, MgResult};

pub fn sanitize_tarball_signature_path(path: &Path) -> MgResult<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => clean.push(part),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => {
                return Err(MgError::Other(format!(
                    "unsafe tar entry path in cached package signature: {}",
                    path.display()
                )));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(MgError::Other(
            "empty tar entry path in cached package signature".to_string(),
        ));
    }
    Ok(clean)
}

pub fn path_to_signature_string(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            std::path::Component::Normal(part) => Some(part.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

pub fn common_tarball_root_prefix(files: &[(String, u64)]) -> Option<String> {
    let mut iter = files.iter().filter_map(|(path, _)| path.split('/').next());
    let first = iter.next()?.to_string();
    if first.is_empty() {
        return None;
    }
    if iter.all(|part| part == first) {
        Some(first)
    } else {
        None
    }
}
