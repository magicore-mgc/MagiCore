//! Create Unix shell configuration directories without following links.
//! Tạo thư mục cấu hình shell Unix mà không đi theo symlink.

use std::path::Path;

#[cfg(not(windows))]
pub(in crate::paths::shell::unix) fn ensure_directory_chain(
    home: &Path,
    directory: &Path,
) -> anyhow::Result<()> {
    let relative = directory
        .strip_prefix(home)
        .map_err(|_| anyhow::anyhow!("shell config directory must be inside the home directory"))?;
    let mut current = home.to_path_buf();
    for component in relative.components() {
        current.push(component);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                anyhow::bail!("shell config directory '{}' is unsafe", current.display())
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&current)?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&current, std::fs::Permissions::from_mode(0o700))?;
                }
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
