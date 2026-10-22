use super::super::is_sensitive;
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::config) fn atomic_write_config_file(
    path: &Path,
    bytes: &[u8],
) -> Result<()> {
    use std::io::Write;

    #[cfg(unix)]
    let contains_credentials = std::str::from_utf8(bytes).is_ok_and(|content| {
        content.lines().any(|line| {
            line.split_once('=')
                .is_some_and(|(key, _)| is_sensitive(key.trim()))
        })
    });
    let existing = match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.file_type().is_file() {
                return Err(crate::error::config_project_file_not_regular(path));
            }
            Some(metadata)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let file_name = path
        .file_name()
        .ok_or_else(crate::error::config_project_file_invalid_path)?;
    let temp = parent.join(format!(
        ".{}.mgc-tmp-{}-{}",
        file_name.to_string_lossy(),
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let result = (|| -> Result<()> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        let mut file = options.open(&temp)?;
        if let Some(metadata) = existing {
            #[cfg(unix)]
            let permissions = if contains_credentials {
                use std::os::unix::fs::PermissionsExt;
                std::fs::Permissions::from_mode(0o600)
            } else {
                metadata.permissions()
            };
            #[cfg(not(unix))]
            let permissions = metadata.permissions();
            file.set_permissions(permissions)?;
        }
        file.write_all(bytes)?;
        file.sync_all()?;
        mgc_lockfile::atomic::atomic_replace_file(&temp, path)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}
