//! Update SQLite and the MessagePack mirror for one package.
//! Cập nhật SQLite và bản sao MessagePack cho một package.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use crate::database::Database;

use super::super::persistence;
use super::super::types::{FileEntry, PackageIndex, package_key};

pub(in crate::index) fn apply(
    db: &Database,
    msgpack_path: &Path,
    packages: &mut HashMap<String, Vec<FileEntry>>,
    id: &mgc_types::PackageId,
    files: Vec<FileEntry>,
) -> Result<()> {
    let sql_files: Vec<(String, String, u64)> = files
        .iter()
        .map(|file| (file.path.clone(), file.blob_hash.clone(), file.size))
        .collect();
    db.replace_package_files(id, &sql_files)?;
    packages.insert(package_key(id), files);

    // Persist a full sorted mirror from the current map (keeps other packages intact).
    let mut records: Vec<PackageIndex> = packages
        .iter()
        .map(|(key, files)| {
            let (id, version) = key.split_once('#').unwrap_or((key.as_str(), ""));
            PackageIndex {
                id: id.to_string(),
                version: version.to_string(),
                files: files.clone(),
            }
        })
        .collect();
    records.sort_by(|left, right| (&left.id, &left.version).cmp(&(&right.id, &right.version)));
    persistence::persist(msgpack_path, &records)
}
