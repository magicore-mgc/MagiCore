use anyhow::{Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub(in crate::commands::tool) fn managed_roots(
    create: bool,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let data = mgc_platform::paths::mgc_data_dir();
    if create {
        fs::create_dir_all(&data)?;
    }
    let data_root = match data.canonicalize() {
        Ok(path) => path,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !create => {
            return Ok((data.clone(), data.join("tools"), data.join("bin")));
        }
        Err(error) => return Err(error.into()),
    };
    if !fs::symlink_metadata(&data_root)?.is_dir() {
        bail!("MGC_DATA_DIR '{}' is not a directory", data_root.display());
    }
    let tools = ensure_child_directory(&data_root, "tools", create)?;
    let bin = ensure_child_directory(&data_root, "bin", create)?;
    Ok((data_root, tools, bin))
}
fn ensure_child_directory(parent: &Path, name: &str, create: bool) -> Result<PathBuf> {
    let path = parent.join(name);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            bail!(
                "managed directory '{}' must be a real directory",
                path.display()
            )
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && create => {
            fs::create_dir(&path)?;
            set_private_directory(&path)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(path),
        Err(error) => return Err(error.into()),
    }
    let canonical = path.canonicalize()?;
    if canonical != path {
        bail!(
            "managed directory '{}' resolves outside its expected path",
            path.display()
        );
    }
    Ok(path)
}
pub(in crate::commands::tool) fn set_private_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}
pub(in crate::commands::tool) fn tool_project_name(package: &str) -> String {
    let hash = blake3::hash(package.as_bytes()).to_hex();
    format!("mgc-tool-{}", &hash[..24])
}
pub(in crate::commands::tool) fn shim_path(bin_root: &Path, binary: &str) -> PathBuf {
    #[cfg(windows)]
    {
        bin_root.join(format!("{binary}.cmd"))
    }
    #[cfg(not(windows))]
    {
        bin_root.join(binary)
    }
}
