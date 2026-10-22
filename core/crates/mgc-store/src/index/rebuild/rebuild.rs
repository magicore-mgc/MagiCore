//! Reconstruct package records from the SQLite source of truth.
//! Dựng các package record từ nguồn dữ liệu chuẩn SQLite.

use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::database::{Database, u64_from_sqlite};

use super::super::types::{FileEntry, PackageIndex};

pub(in crate::index) fn from_database(db: &Database) -> Result<Vec<PackageIndex>> {
    let mut stmt = db
        .conn()
        .prepare(
            "SELECT id, version, path, blob_hash, size FROM package_files ORDER BY id, version, path",
        )
        .context("prepare package_files scan")?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                u64_from_sqlite(row.get(4)?, 4)?,
            ))
        })
        .context("scan package_files")?;

    let mut order: Vec<(String, String)> = Vec::new();
    let mut by_package: HashMap<String, Vec<FileEntry>> = HashMap::new();
    for row in rows {
        let (id, version, path, blob_hash, size) = row?;
        let key = format!("{id}#{version}");
        if !by_package.contains_key(&key) {
            order.push((id, version));
        }
        by_package.entry(key).or_default().push(FileEntry {
            path,
            blob_hash,
            size,
        });
    }

    Ok(order
        .into_iter()
        .map(|(id, version)| {
            let key = format!("{id}#{version}");
            PackageIndex {
                id,
                version,
                files: by_package.remove(&key).unwrap_or_default(),
            }
        })
        .collect())
}
