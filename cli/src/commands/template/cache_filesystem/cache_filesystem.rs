use super::error;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub(in crate::commands::template) fn ensure_template_cache_parent(parent: &Path) -> Result<()> {
    use std::path::Component;
    let mut current = PathBuf::new();
    for component in parent.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                current.push(component.as_os_str());
                continue;
            }
            Component::CurDir => continue,
            Component::ParentDir => {
                return Err(error::template_cache_unsafe_path(parent));
            }
            Component::Normal(part) => current.push(part),
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
                return Err(error::template_cache_unsafe_path(&current));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&current)?,
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

/// Reject links and Windows junction/reparse objects before cache mutation.
/// Chặn cả junction/reparse Windows, không chỉ symlink thông thường.
pub(crate) fn template_cache_is_link(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    false
}
