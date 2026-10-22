//! Install behavior and security controls passed across the adapter contract. — Tùy chọn hành vi install và kiểm soát bảo mật truyền qua hợp đồng adapter.

use crate::package::PackageId;
/// Options controlling install behaviour.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct InstallOptions {
    /// Skip running lifecycle scripts (preinstall / install / postinstall).
    pub ignore_scripts: bool,
    /// Explicitly allow lifecycle scripts. Defaults to false for secure installs.
    pub allow_scripts: bool,
    /// Use flat hoisting layout instead of strict symlink virtual store.
    pub legacy_flat: bool,
    /// Fail fast if mgc.lock is missing or out-of-sync (CI mode).
    pub frozen: bool,
    /// The orchestrator verified this graph from the current authoritative lock.
    /// Graph đã được kiểm từ lock hiện hành; materialize không được ghi lại lock.
    pub replay_from_lock: bool,
    /// Skip checking already-installed root packages. Only materialize what differs.
    /// Safe after add/remove when graph changed by exactly one leaf package.
    pub incremental: bool,
    /// Packages to force-install even when incremental mode is active.
    /// Only used when `incremental` is true.
    pub force_install: Vec<PackageId>,
    /// Prefer reusing installed versions (dedupe) instead of latest (02 §2.1).
    /// Opt-in — default off for safety.
    pub prefer_dedupe: bool,
    /// Re-link dangling symlinks in node_modules from the virtual store (02 §2.2).
    pub repair: bool,
    /// Offline install: resolve from lockfile + local cache only; any required
    /// network fetch fails closed instead of silently hitting the registry.
    /// Cài offline: chỉ dùng lockfile + cache local; thiếu cache thì fail rõ.
    pub offline: bool,
}
