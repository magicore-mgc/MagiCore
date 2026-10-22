//! Shared Node-family package-manager lockfile selection.
use super::super::PackageManager;
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_package_manager(
    project_root: &Path,
) -> PackageManager {
    // Check lockfiles to determine package manager — kiểm tra lockfile để xác định package manager
    if project_root.join("deno.lock").exists() {
        PackageManager::Deno
    } else if project_root.join("pnpm-lock.yaml").exists() {
        PackageManager::Pnpm
    } else if project_root.join("yarn.lock").exists() {
        PackageManager::Yarn
    } else if project_root.join("bun.lockb").exists() {
        PackageManager::Bun
    } else if project_root.join("package-lock.json").exists() {
        PackageManager::Npm
    } else {
        // Default to npm if no lockfile — mặc định npm nếu không có lockfile
        PackageManager::Npm
    }
}
