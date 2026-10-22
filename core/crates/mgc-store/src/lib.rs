#![cfg_attr(test, allow(clippy::unwrap_used))]
//! Package store and cache management for MagiCore
//!
//! Content-addressable storage, package caching, and tarball extraction.

pub mod cache;
pub mod cas;
pub mod database;
pub mod failpoint;
pub mod index;
pub mod layout;

pub use cache::PackageCache;
pub use cas::{
    COMPILED_CACHE_SCHEMA_VERSION, CompilationKey, CompiledCache, CompiledModule, ContentStore,
    IntegrityHash,
};
pub use database::{
    CasGenerationError, CasIntegrityCounts, CasIntegritySnapshotError, Database, DatabaseEntry,
    ProjectInstallLock, StagingLease,
};
pub use index::{FileEntry, StoreIndex};
pub use layout::Layout;

/// Trả về đường dẫn global store: `$MGC_DATA_DIR/store/v3`
/// (env `MAGICORE_STORE_ROOT` override — tests + custom setups)
pub fn default_store_root() -> std::path::PathBuf {
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(override_dir) = std::env::var("MAGICORE_STORE_ROOT")
        && !override_dir.is_empty()
    {
        return std::path::PathBuf::from(override_dir);
    }
    mgc_platform::paths::mgc_data_dir().join("store").join("v3")
}
