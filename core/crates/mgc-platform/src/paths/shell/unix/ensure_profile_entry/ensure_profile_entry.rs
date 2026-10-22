//! Validate, read and update one Unix shell profile.
//! Kiểm tra, đọc và cập nhật một profile shell Unix.

use std::path::Path;

use super::super::atomic_replace_profile::atomic_replace_profile;
use super::super::managed_path_profile_block::{
    managed_path_profile_block, replace_managed_profile_block,
};

#[cfg(not(windows))]
pub(in crate::paths::shell::unix) fn ensure_profile_entry(
    profile: &Path,
    bin: &Path,
    fish: bool,
) -> anyhow::Result<()> {
    use std::io::Read;

    const MAX_PROFILE_BYTES: u64 = 1024 * 1024;
    let metadata = match std::fs::symlink_metadata(profile) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            anyhow::bail!(
                "shell profile '{}' must not be a symlink",
                profile.display()
            )
        }
        Ok(metadata) if !metadata.is_file() => {
            anyhow::bail!(
                "shell profile '{}' must be a regular file",
                profile.display()
            )
        }
        Ok(metadata) => Some(metadata),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    let mut current = String::new();
    if let Some(metadata) = &metadata {
        if metadata.len() > MAX_PROFILE_BYTES {
            anyhow::bail!(
                "shell profile '{}' exceeds the 1 MiB safety limit",
                profile.display()
            );
        }
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        options
            .open(profile)?
            .take(MAX_PROFILE_BYTES + 1)
            .read_to_string(&mut current)?;
        if current.len() as u64 > MAX_PROFILE_BYTES {
            anyhow::bail!(
                "shell profile '{}' exceeds the 1 MiB safety limit",
                profile.display()
            );
        }
    }
    let desired = managed_path_profile_block(bin, fish)?;
    let updated = replace_managed_profile_block(&current, &desired)?;
    if updated == current {
        return Ok(());
    }
    atomic_replace_profile(profile, updated.as_bytes(), metadata.as_ref())
}
