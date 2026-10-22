//! PackageAdapter implementation for game cores.
//! Detects Bevy/Godot/Unity/Unreal without claiming an unimplemented
//! MagiCore-owned dependency engine.
//!
//! Dependency operations fail closed until a MagiCore-owned resolver,
//! lock, fetcher, store, and materializer exist for the selected engine.
//! No compatibility package-manager execution is provided by this adapter.
//! Adapter này không cung cấp đường chạy package manager tương thích.

mod adapter;
mod audit_provider;
mod capability_manifest;
mod content_store_provider;
mod dependency_resolver;
mod factory;
mod identity;
mod lockfile_provider;
mod package_adapter;
mod project_detection;
mod scaffold_provider;
mod unclaimed_capabilities;

pub use self::adapter::GameAdapter;
pub use self::factory::adapter_for;
