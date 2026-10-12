#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Cache module tests for app adapter.

use mgc_app_adapter::AppLanguage;
use mgc_app_adapter::cache::{cache_dir, cache_size, clear_cache};

#[test]
fn flutter_cache_dir_points_to_mgc_store() {
    let dir = cache_dir(AppLanguage::Flutter).unwrap();
    assert_eq!(dir, mgc_platform::paths::mgc_data_dir().join("store/pub"));
}

#[test]
fn app_lanes_without_native_cache_ownership_are_not_managed() {
    assert!(cache_dir(AppLanguage::Kotlin).is_err());
    assert!(cache_dir(AppLanguage::ReactNative).is_err());
    assert!(cache_dir(AppLanguage::ObjC).is_err());
}

#[test]
fn swift_cache_dir_platform_specific() {
    let dir = cache_dir(AppLanguage::Swift).unwrap();
    assert_eq!(dir, mgc_platform::paths::mgc_data_dir().join("store/swift"));
}

#[test]
fn external_dotnet_and_rust_caches_are_not_exposed_as_mgc_managed_cache() {
    assert!(cache_dir(AppLanguage::Maui).is_err());
    assert!(cache_dir(AppLanguage::Tauri).is_err());
}

#[test]
fn cache_size_handles_nonexistent() {
    let result = cache_size(AppLanguage::Flutter);
    // size is u64 — any value valid; contract: no panic
    // Gọi được là đạt — Err chấp nhận khi cache không truy cập được
    // (must simply not panic; Err acceptable when cache inaccessible)
    drop(result);
}

#[test]
fn clear_cache_doesnt_panic() {
    let _ = clear_cache(AppLanguage::Multi);
}
