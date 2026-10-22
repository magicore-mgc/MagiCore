// manifest_io owner — manifest_io owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::manifest_path::ManifestPathError;
use super::super::manifest_path::{existing_or_encoded_manifest_path, model_manifest_dir};
use super::super::manifest_types::ModelManifest;
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(in crate::commands::model) fn save_manifest(m: &ModelManifest) -> Result<()> {
    save_manifest_in(model_manifest_dir(), m)
}

pub(in crate::commands::model) fn save_manifest_in(dir: PathBuf, m: &ModelManifest) -> Result<()> {
    let relative = existing_or_encoded_manifest_path(&dir, &m.name)?;
    std::fs::create_dir_all(&dir)?;
    if !std::fs::symlink_metadata(&dir)?.file_type().is_dir() {
        return Err(ManifestPathError::UnsafeManifestPath.into());
    }
    let mut parent = dir.clone();
    for component in relative.parent().into_iter().flat_map(Path::components) {
        parent.push(component);
        match std::fs::symlink_metadata(&parent) {
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => return Err(ManifestPathError::UnsafeManifestPath.into()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::create_dir(&parent) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        if !std::fs::symlink_metadata(&parent)?.file_type().is_dir() {
                            return Err(ManifestPathError::UnsafeManifestPath.into());
                        }
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) => return Err(error.into()),
        }
    }
    let path = dir.join(relative);
    let staging = path.with_file_name(format!(
        ".{}.mgc-manifest-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("model.json"),
        uuid::Uuid::new_v4()
    ));
    let mut staging_created = false;
    let result = (|| -> Result<()> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staging)?;
        staging_created = true;
        file.write_all(&serde_json::to_vec_pretty(m)?)?;
        file.sync_all()?;
        drop(file);
        mgc_lockfile::atomic::atomic_replace_file(&staging, &path)?;
        #[cfg(unix)]
        if let Some(parent) = path.parent() {
            std::fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    })();
    if result.is_err() && staging_created {
        let _ = std::fs::remove_file(&staging);
    }
    result
}

pub(in crate::commands::model) fn read_manifests_in(dir: PathBuf) -> Vec<ModelManifest> {
    let mut out = Vec::new();
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        if !std::fs::symlink_metadata(&d).is_ok_and(|metadata| metadata.file_type().is_dir()) {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in entries.flatten() {
            let p = e.path();
            let Ok(metadata) = std::fs::symlink_metadata(&p) else {
                continue;
            };
            if metadata.file_type().is_dir() {
                stack.push(p);
            } else if metadata.file_type().is_file() && p.extension().is_some_and(|x| x == "json") {
                // Edition 2024 let-chain keeps manifest parsing conditions together.
                // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
                if let Ok(s) = std::fs::read_to_string(&p)
                    && let Ok(m) = serde_json::from_str(&s)
                {
                    out.push(m);
                }
            }
        }
    }
    out
}

pub(in crate::commands::model) fn now_iso() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}
