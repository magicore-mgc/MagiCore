//! StoreIndex public API and operation coordinator.
//! API công khai StoreIndex và điều phối thao tác.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use crate::database::Database;
use crate::layout::Layout;
use mgc_types::PackageId;

use super::persistence;
use super::prune;
use super::rebuild;
use super::types::{FileEntry, package_key};
use super::upsert;

pub struct StoreIndex {
    db: Database,
    msgpack_path: std::path::PathBuf,
    /// id#version -> files (mirrors SQLite, loaded from msgpack at open).
    packages: HashMap<String, Vec<FileEntry>>,
}

impl StoreIndex {
    pub fn open(layout: &Layout, db: Database) -> Result<Self> {
        let msgpack_path = layout.index_msgpack_path();
        let mut index = Self {
            db,
            msgpack_path,
            packages: HashMap::new(),
        };
        index.load_or_rebuild()?;
        Ok(index)
    }

    fn load_or_rebuild(&mut self) -> Result<()> {
        if let Some(packages) = persistence::load(&self.msgpack_path)? {
            self.packages = persistence::packages_to_map(&packages);
            return Ok(());
        }
        // Missing / corrupt / stale version -> rebuild from SQLite.
        self.rebuild()
    }

    /// Rebuild the msgpack file from SQLite (source of truth).
    pub fn rebuild(&mut self) -> Result<()> {
        let packages = rebuild::from_database(&self.db)?;
        self.packages = persistence::packages_to_map(&packages);
        persistence::persist(&self.msgpack_path, &packages)
    }

    /// Upsert one package's file listing (SQLite + msgpack in lockstep).
    pub fn upsert_package_files(&mut self, id: &PackageId, files: Vec<FileEntry>) -> Result<()> {
        upsert::apply(&self.db, &self.msgpack_path, &mut self.packages, id, files)
    }

    /// Full file listing for a package — None = not indexed.
    pub fn verify_package(&self, id: &PackageId) -> Option<&[FileEntry]> {
        self.packages.get(&package_key(id)).map(Vec::as_slice)
    }

    /// Blob hashes currently referenced by any indexed package (from SQLite —
    /// source of truth, even if msgpack is stale).
    pub fn list_blob_hashes(&self) -> Result<Vec<String>> {
        self.db.list_all_blob_hashes()
    }

    /// Prune CAS blobs not referenced by any indexed package.
    ///
    /// Safe-guards (fail-closed): only files older than `max_age` are removed,
    /// and only when (a) the path lives under the CAS blob layout AND (b) the
    /// blob hash is unknown to the index AND (c) the file has no external
    /// hardlinks (nlink <= 1). If the index holds no data at all while blobs
    /// exist (index not yet built), this returns an error instead of deleting.
    pub fn prune_blobs(&self, cas_root: &Path, max_age: std::time::Duration) -> Result<usize> {
        prune::prune_blobs(&self.db, &self.packages, cas_root, max_age)
    }
}
