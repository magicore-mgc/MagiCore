use super::error;
use crate::commands::template::{ensure_template_cache_parent, template_cache_is_link};
use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile;

/// Extract tarball entries vào target, bỏ segment đầu của entry name
/// (npm tarball prefix `mgc-create-.../...`).
pub(crate) fn extract_tarball_into(
    bytes: &[u8],
    target: &Path,
    version: Option<&str>,
) -> Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| error::template_cache_unsafe_path(target))?;
    ensure_template_cache_parent(parent)?;
    let previous = match fs::symlink_metadata(target) {
        Ok(metadata) if template_cache_is_link(&metadata) || !metadata.is_dir() => {
            return Err(error::template_cache_unsafe_path(target));
        }
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => return Err(error.into()),
    };

    // Validate the complete archive in a private same-filesystem staging tree.
    // Giữ cache đang dùng nguyên vẹn cho tới khi toàn archive đã qua extractor an toàn.
    let staging = tempfile::tempdir_in(parent)?;
    let unpacked = staging.path().join("unpacked");
    mgc_resolver::protocols::archive::extract_tar_gz(bytes, &unpacked)?;
    let entries = fs::read_dir(&unpacked)?.collect::<std::io::Result<Vec<_>>>()?;
    if entries.len() != 1 || !entries[0].file_type()?.is_dir() {
        return Err(error::template_archive_layout_invalid());
    }
    let packaged = entries[0].path();
    if !packaged.join("template.toml").is_file() || !packaged.join("sources").is_dir() {
        return Err(error::template_archive_layout_invalid());
    }
    crate::scaffold::processor::validate_template_manifest(&packaged)?;
    if let Some(version) = version {
        semver::Version::parse(version).map_err(|_| error::template_metadata_invalid())?;
        fs::write(packaged.join(".mgc-version"), version)?;
    }

    let backup = staging.path().join("previous");
    if previous {
        fs::rename(target, &backup)?;
    }
    if let Err(error) = fs::rename(&packaged, target) {
        if previous {
            // Preserve the backup if rollback itself fails; never delete the old cache.
            // Nếu rollback lỗi, giữ backup trên đĩa thay vì để TempDir xóa cache cũ.
            if let Err(rollback) = fs::rename(&backup, target) {
                let retained = staging.keep();
                return Err(error::template_cache_rollback_failed(
                    target, &retained, &error, &rollback,
                ));
            }
        }
        return Err(error.into());
    }
    staging.close()?;
    Ok(())
}
