#![cfg_attr(test, allow(clippy::unwrap_used))]

//! `adapters/web/src/lib.rs` — public facade for the Web ecosystem adapter.
//!
//! The concrete adapter state and behavior are owned by `adapter/`; ecosystem
//! services remain in their existing modules.

mod adapter;
pub use adapter::WebAdapter;

pub mod audit;
pub mod bundler;
pub mod cache;
pub mod cache_daemon;
pub mod cache_metadata;
pub mod cache_prune;
pub mod install;
pub mod layout;
pub mod lifecycle;
pub mod list;
pub mod lockfile;
pub mod manifest;
pub mod native;
pub mod prefetch;
pub mod profile;
pub mod provider;
pub mod registry_config;
pub mod resolution_cache;
pub mod sbom;
pub mod supply_chain;
pub mod update;

#[cfg(test)]
#[path = "test/unit_tests/mod.rs"]
mod tests;

#[cfg(test)]
#[path = "test/script_policy_tests.rs"]
mod script_policy_tests;

/// Process-wide environment lock for parallel tests that mutate env vars, or a
/// poisoning-free guard. A test that panics while holding the lock must not
/// fail every later test in the same binary, so acquisition always recovers.
/// (Lock môi trường dùng chung cho test song song đổi env, hoặc guard không
/// nhiễm độc. Test panic khi giữ lock không được làm hỏng mọi test sau, nên
/// mọi lần acquire đều khôi phục.)
#[cfg(test)]
pub(crate) fn environment_test_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

#[cfg(test)]
pub(crate) fn environment_test_guard() -> std::sync::MutexGuard<'static, ()> {
    environment_test_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub use lockfile::{read_web_lockfile, read_web_lockfile_checked};
pub use manifest::PackageJson;
pub use prefetch::spawn_tarball_download;
pub use registry_config::{
    DEFAULT_NPM_REGISTRY, effective_registry_url, validate_registry_allowed,
};
pub use resolution_cache::manifest_resolution_cache_key;
pub use sbom::generate_sbom;

// Crate-local compatibility paths used by the existing test modules.
pub(crate) use adapter::resolution::compat::{
    exact_version_from_range, is_manifest_root_package, manifest_with_resolved_specs,
    normalize_resolved_graph, select_resolved_candidate,
};
