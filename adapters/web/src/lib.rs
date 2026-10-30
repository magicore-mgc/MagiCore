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

/// Run a test inside its own project directory.
/// Chạy một test trong thư mục project riêng của nó.
///
/// The default release-age quarantine reads the process project
/// (`current_dir`) on a first resolution, exactly like `mgc install`. A test
/// that resolves without a project scope would be gated by the default
/// quarantine against a fixture that has no publish timestamps, so every such
/// test states its project explicitly instead of relying on the harness cwd.
/// (Quarantine mặc định đọc project của process khi resolve lần đầu, giống
/// `mgc install`. Test resolve không có scope project sẽ bị quarantine mặc
/// định chặn với fixture không có timestamp, nên mỗi test phải nêu rõ
/// project của nó.)
#[cfg(test)]
pub(crate) async fn with_project_dir<T>(
    root: &std::path::Path,
    run: impl std::future::Future<Output = T>,
) -> T {
    // Serialize cwd changes: the process has exactly one current directory.
    // An async mutex is required because the guard spans an await point.
    // (Tuần tự hóa thay đổi cwd: process chỉ có một current directory. Phải
    // dùng async mutex vì guard được giữ qua điểm await.)
    static CWD_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    let _guard = CWD_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await;
    let previous = std::env::current_dir().expect("test harness has a current directory");
    std::env::set_current_dir(root).expect("enter the test project directory");
    use futures_util::FutureExt;
    let outcome = std::panic::AssertUnwindSafe(run).catch_unwind().await;
    let restore = std::env::set_current_dir(&previous);
    assert!(
        restore.is_ok(),
        "failed to restore the test current directory"
    );
    match outcome {
        Ok(value) => value,
        Err(panic) => std::panic::resume_unwind(panic),
    }
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
