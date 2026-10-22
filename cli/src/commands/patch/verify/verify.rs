//! Verify patch path containment, file type, size and digest. / Xác minh containment, loại file, size và digest patch.

use anyhow::{Result, bail};
use mgc_config::project::ProjectConfig;
use mgc_resolver::patches::get_patches_dir;
use std::fs;
use std::path::{Component, Path, PathBuf};

use super::super::config::MAX_PATCH_VERIFY_BYTES;

pub(in crate::commands::patch) async fn verify_patches(
    project: &ProjectConfig,
    project_root: &Path,
) -> Result<()> {
    let patches_dir = get_patches_dir(Some(project_root))?;
    let mut all_ok = true;

    for spec in &project.patches {
        let patch_path = resolve_project_patch_path(project_root, &patches_dir, &spec.patch_path)?;
        match verify_patch_file_no_follow(&patch_path, &spec.integrity) {
            Ok(true) => println!("✓ {}: integrity OK", spec.package),
            Ok(false) => {
                println!("✗ {}: integrity MISMATCH", spec.package);
                all_ok = false;
            }
            Err(e) => {
                println!("✗ {}: error verifying — {}", spec.package, e);
                all_ok = false;
            }
        }
    }

    if !all_ok {
        bail!("Some patches failed verification");
    }
    println!("All patches verified");
    Ok(())
}

fn resolve_project_patch_path(
    project_root: &Path,
    patches_dir: &Path,
    patch_name: &str,
) -> Result<PathBuf> {
    if patch_name.is_empty()
        || patch_name.contains('/')
        || patch_name.contains('\\')
        || Path::new(patch_name)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(crate::error::patch_path_rejected(
            "path must be one filename inside the project patch store",
        ));
    }

    let project_root = project_root.canonicalize()?;
    let magicore_dir = project_root.join(".magicore");
    let metadata = fs::symlink_metadata(&magicore_dir)?;
    if !metadata.file_type().is_dir()
        || mgc_lockfile::project_lock::path_is_link_or_reparse(&magicore_dir)
    {
        return Err(crate::error::patch_path_rejected(
            "project metadata directory is symlinked or invalid",
        ));
    }

    let canonical_patches = patches_dir.canonicalize()?;
    let expected_patches = magicore_dir.join("patches");
    if canonical_patches != expected_patches {
        return Err(crate::error::patch_path_rejected(
            "project patch store resolves outside its expected directory",
        ));
    }
    let metadata = fs::symlink_metadata(patches_dir)?;
    if !metadata.file_type().is_dir()
        || mgc_lockfile::project_lock::path_is_link_or_reparse(patches_dir)
    {
        return Err(crate::error::patch_path_rejected(
            "project patch directory is symlinked or invalid",
        ));
    }

    let patch_path = patches_dir.join(patch_name);
    let metadata = fs::symlink_metadata(&patch_path)?;
    if !metadata.file_type().is_file()
        || mgc_lockfile::project_lock::path_is_link_or_reparse(&patch_path)
    {
        return Err(crate::error::patch_path_rejected(
            "patch target is symlinked or not a regular file",
        ));
    }
    Ok(patch_path)
}

fn verify_patch_file_no_follow(path: &Path, expected: &str) -> Result<bool> {
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options.open(path)?;
    use std::io::Read;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(crate::error::patch_path_rejected(
            "patch target is not a regular file",
        ));
    }
    if metadata.len() > MAX_PATCH_VERIFY_BYTES {
        return Err(crate::error::patch_path_rejected(
            "patch file exceeds the 64 MiB verification limit",
        ));
    }
    let mut reader = file.take(MAX_PATCH_VERIFY_BYTES + 1);
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut total = 0u64;
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total += read as u64;
        if total > MAX_PATCH_VERIFY_BYTES {
            return Err(crate::error::patch_path_rejected(
                "patch file grew beyond the 64 MiB verification limit while reading",
            ));
        }
        hasher.update(&buffer[..read]);
    }
    let actual = hex::encode(hasher.finalize());
    let expected = expected.strip_prefix("sha256-").unwrap_or(expected);
    Ok(actual == expected)
}
