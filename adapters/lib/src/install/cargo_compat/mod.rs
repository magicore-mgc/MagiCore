//! Cargo compatibility projection owner modules.
//! Các module phụ trách projection tương thích Cargo.

mod atomic_write;
mod build;
mod cargo_compat;
mod lock;
mod manifest;
mod package_graph;
mod projection;
mod source_checksum;
mod types;

pub use cargo_compat::{validate_build, validate_build_for_core, verify_managed_source};
pub(crate) use lock::{refresh_lock, validate_install};
pub(crate) use source_checksum::{verify_source_checksum, write_source_checksum};

pub(super) use atomic_write::atomic_write;
pub(super) use build::validate_build_for_core_impl;
pub(super) use manifest::{read_project_manifest, resolve_manifest_roots};
pub(super) use package_graph::{
    cargo_dependency_label, package_id, resolve_locked_dependency, rust_packages, rust_root_ids,
    validate_build_ownership,
};
pub(super) use projection::project_cargo_lock;
pub(super) use types::{
    CARGO_LOCK_HEADER, CARGO_LOCK_VERSION, CRATES_IO_SOURCE, CargoChecksum, CargoLock,
    CargoPackage, CargoProjectManifest,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
