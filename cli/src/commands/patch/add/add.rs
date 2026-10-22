//! Add a patch after bounded reading and safe publication. / Thêm patch sau khi đọc giới hạn và ghi an toàn.

use anyhow::{Result, bail};
use mgc_config::project::ProjectConfig;
use mgc_resolver::patches::get_patches_dir;
use mgc_types::PatchSpec;
use std::fs;
use std::path::Path;

use super::super::config::MAX_PATCH_VERIFY_BYTES;

pub(in crate::commands::patch) async fn add_patch(
    project: &mut ProjectConfig,
    project_root: &Path,
    package: &str,
    file: &str,
    range: Option<String>,
    lock: &mgc_lockfile::project_lock::ProjectWriteLock,
) -> Result<()> {
    let package = mgc_types::PackageName::new(package.to_string())?;
    let version_range = range
        .map(|range| mgc_types::VersionRange::parse(&range))
        .transpose()?
        .unwrap_or_else(mgc_types::VersionRange::star);
    if project
        .patches
        .iter()
        .any(|patch| patch.package == package.as_str() && patch.version_range == version_range)
    {
        return Err(crate::error::patch_duplicate_identity());
    }

    let content = read_patch_source_bounded(Path::new(file))?;
    let integrity = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(&content);
        format!("sha256-{}", hex::encode(h.finalize()))
    };

    // Copy patch to patches dir
    let patches_dir = get_patches_dir(Some(project_root))?;
    ensure_patch_directory(project_root, &patches_dir)?;
    let mut identity = sha2::Sha256::new();
    use sha2::Digest;
    identity.update(package.as_str().as_bytes());
    identity.update([0]);
    identity.update(version_range.as_str().as_bytes());
    let dest_name = format!("patch-{}.patch", hex::encode(identity.finalize()));
    let dest = patches_dir.join(&dest_name);
    mgc_lockfile::atomic::atomic_write_locked(
        lock,
        &dest,
        &content,
        std::time::Duration::from_secs(60),
    )?;

    // Add to project patches
    let spec = PatchSpec::new(package.to_string(), version_range, dest_name, integrity);

    // Store in mgc.toml [patches]
    project.patches.push(spec);
    project.save(project_root)?;

    println!(
        "Recorded patch configuration for {} -> {}; install will fail closed until patch application is supported",
        package,
        dest.display()
    );
    Ok(())
}

fn read_patch_source_bounded(path: &Path) -> Result<Vec<u8>> {
    use std::io::Read;
    let file = fs::File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(crate::error::patch_path_rejected(
            "patch source is not a regular file",
        ));
    }
    if file.metadata()?.len() > MAX_PATCH_VERIFY_BYTES {
        return Err(crate::error::patch_path_rejected(
            "patch source exceeds the 64 MiB limit",
        ));
    }
    let mut content = Vec::new();
    file.take(MAX_PATCH_VERIFY_BYTES + 1)
        .read_to_end(&mut content)?;
    if content.len() as u64 > MAX_PATCH_VERIFY_BYTES {
        return Err(crate::error::patch_path_rejected(
            "patch source grew beyond the 64 MiB limit while reading",
        ));
    }
    Ok(content)
}

fn ensure_patch_directory(project_root: &Path, patches_dir: &Path) -> Result<()> {
    let expected = project_root.join(".magicore").join("patches");
    if patches_dir != expected {
        bail!(
            "refusing unexpected project patch directory: {}",
            patches_dir.display()
        );
    }
    if mgc_lockfile::project_lock::path_is_link_or_reparse(&expected) {
        bail!(
            "refusing symlink/reparse project patch directory: {}",
            expected.display()
        );
    }
    match fs::symlink_metadata(&expected) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => bail!(
            "project patch path is not a real directory: {}",
            expected.display()
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&expected)?;
            let metadata = fs::symlink_metadata(&expected)?;
            if metadata.file_type().is_dir()
                && !mgc_lockfile::project_lock::path_is_link_or_reparse(&expected)
            {
                Ok(())
            } else {
                bail!(
                    "project patch path changed during creation: {}",
                    expected.display()
                )
            }
        }
        Err(error) => Err(error.into()),
    }
}
