// Only the Windows shim branch calls atomic_replace_file.
// (Chỉ nhánh shim Windows gọi atomic_replace_file.)
#[cfg(windows)]
use super::super::atomic_file::atomic_replace_file;
use super::super::types::ToolRecord;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub(in crate::commands::tool) fn ensure_shim(
    project_root: &Path,
    record: &ToolRecord,
    shim: &Path,
    replace_owned: bool,
) -> Result<()> {
    let bin_dir = project_root.join("node_modules/.bin");
    let executable = shimmed_binary_path(&bin_dir, &record.binary)?;
    let root = project_root.canonicalize()?;
    let local_target = executable.canonicalize()?;
    if !local_target.starts_with(&root) || !local_target.is_file() {
        bail!("tool shim target is outside the managed project");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&local_target)?;
        if metadata.permissions().mode() & 0o111 == 0 {
            bail!(
                "package executable '{}' is not marked executable",
                local_target.display()
            );
        }
    }

    #[cfg(unix)]
    {
        match fs::symlink_metadata(shim) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let existing = fs::read_link(shim)?;
                if existing == executable {
                    return Ok(());
                }
                if !replace_owned {
                    bail!(
                        "global executable '{}' already exists; refusing to overwrite it",
                        shim.display()
                    );
                }
                bail!(
                    "global executable '{}' is no longer owned by this tool; refusing to replace it",
                    shim.display()
                );
            }
            Ok(_) => bail!(
                "global executable '{}' already exists; refusing to overwrite it",
                shim.display()
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        std::os::unix::fs::symlink(&executable, shim)?;
    }
    #[cfg(windows)]
    {
        // Use the package's declared target, not the generated node_modules/.bin
        // `.cmd` shim. Passing a `.cmd` file to `node` creates an invalid nested shim.
        // Dùng target khai báo của package, không dùng shim `.cmd` trong .bin;
        // truyền file `.cmd` cho `node` sẽ tạo shim lồng sai.
        let package_root = project_root.join("node_modules").join(&record.package);
        let package_target = package_root.join(&record.target).canonicalize()?;
        if !package_target.starts_with(package_root.canonicalize()?) || !package_target.is_file() {
            bail!("package executable target is outside the installed package");
        }
        let contents = windows_shim_contents(project_root, record)?;
        if shim.exists() && !replace_owned {
            bail!(
                "global executable '{}' already exists; refusing to overwrite it",
                shim.display()
            );
        }
        atomic_replace_file(shim, contents.as_bytes())?;
    }
    Ok(())
}
fn shimmed_binary_path(bin_dir: &Path, binary: &str) -> Result<PathBuf> {
    #[cfg(windows)]
    let path = bin_dir.join(format!("{binary}.cmd"));
    #[cfg(not(windows))]
    let path = bin_dir.join(binary);
    let metadata =
        fs::symlink_metadata(&path).context("package does not expose the selected executable")?;
    if !metadata.file_type().is_symlink() && !metadata.is_file() {
        bail!(
            "package executable shim '{}' is not a regular file or symlink",
            path.display()
        );
    }
    Ok(path)
}
pub(in crate::commands::tool) fn remove_owned_shim(
    project_root: &Path,
    record: &ToolRecord,
    shim: &Path,
) -> Result<()> {
    match fs::symlink_metadata(shim) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
        Ok(_) => {}
    }
    #[cfg(unix)]
    {
        let expected =
            shimmed_binary_path(&project_root.join("node_modules/.bin"), &record.binary)?;
        let actual = fs::read_link(shim)?;
        if actual != expected {
            bail!(
                "global executable '{}' is not owned by this tool; refusing to remove it",
                shim.display()
            );
        }
        fs::remove_file(shim)?;
    }
    #[cfg(windows)]
    {
        let expected = windows_shim_contents(project_root, record)?;
        let actual = mgc_config::project::read_regular_project_text(shim, "managed tool shim")?
            .ok_or_else(|| anyhow::anyhow!("managed tool shim disappeared"))?;
        if actual != expected {
            bail!(
                "global executable '{}' is not owned by this tool; refusing to remove it",
                shim.display()
            );
        }
        fs::remove_file(shim)?;
    }
    Ok(())
}
#[cfg(windows)]
fn windows_shim_contents(project_root: &Path, record: &ToolRecord) -> Result<String> {
    let executable = project_root
        .join("node_modules")
        .join(&record.package)
        .join(&record.target)
        .canonicalize()?;
    render_windows_shim_contents(&executable)
}
#[cfg(any(windows, test))]
pub(in crate::commands::tool) fn render_windows_shim_contents(executable: &Path) -> Result<String> {
    let path_text = executable.to_string_lossy();
    if path_text.contains('%') || path_text.chars().any(char::is_control) {
        bail!("Windows tool path contains characters unsafe for a command shim");
    }
    let command = if executable
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("exe"))
    {
        format!("\"{}\" %*", path_text)
    } else {
        format!("node \"{}\" %*", path_text)
    };
    Ok(format!("@echo off\r\n{command}\r\n"))
}
