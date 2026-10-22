//! MessagePack load, checksum, map conversion, and persistence.
//! Đọc MessagePack, checksum, chuyển map và lưu chỉ mục.

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use blake3::Hasher;

use super::super::types::{FileEntry, IndexFile, PackageIndex};

const INDEX_FORMAT_VERSION: u32 = 1;

pub(in crate::index) fn load(path: &Path) -> Result<Option<Vec<PackageIndex>>> {
    if !path.exists() {
        return Ok(None);
    }

    let raw =
        std::fs::read(path).with_context(|| format!("read index msgpack {}", path.display()))?;
    match rmp_serde::from_slice::<IndexFile>(&raw) {
        Ok(file) if file.version == INDEX_FORMAT_VERSION => {
            let checksum = checksum_of_packages(&file.packages);
            if checksum == file.checksum {
                Ok(Some(file.packages))
            } else {
                Ok(None)
            }
        }
        Ok(_) | Err(_) => Ok(None),
    }
}

pub(in crate::index) fn persist(path: &Path, packages: &[PackageIndex]) -> Result<()> {
    let file = IndexFile {
        version: INDEX_FORMAT_VERSION,
        checksum: checksum_of_packages(packages),
        packages: packages.to_vec(),
    };
    let data = rmp_serde::to_vec(&file).context("encode index msgpack")?;
    atomic_write(path, &data)
}

pub(in crate::index) fn packages_to_map(
    packages: &[PackageIndex],
) -> HashMap<String, Vec<FileEntry>> {
    packages
        .iter()
        .map(|package| {
            (
                format!("{}#{}", package.id, package.version),
                package.files.clone(),
            )
        })
        .collect()
}

fn checksum_of_packages(packages: &[PackageIndex]) -> String {
    let mut hasher = Hasher::new();
    for package in packages {
        hasher.update(package.id.as_bytes());
        hasher.update(package.version.as_bytes());
        for file in &package.files {
            hasher.update(file.path.as_bytes());
            hasher.update(file.blob_hash.as_bytes());
            hasher.update(&file.size.to_le_bytes());
        }
    }
    hasher.finalize().to_hex().to_string()
}

fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let tmp = path.with_extension("msgpack.tmp");
    std::fs::write(&tmp, data).with_context(|| format!("write tmp {}", tmp.display()))?;
    std::fs::rename(&tmp, path).with_context(|| format!("rename to {}", path.display()))?;
    Ok(())
}
