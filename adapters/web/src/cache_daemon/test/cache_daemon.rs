//! Process-local cache behavior tests. / Test hành vi cache theo process.

#![allow(clippy::unwrap_used)]

use crate::cache_daemon::MemoryCacheDaemon;
use mgc_types::{PackageId, PackageName, Version};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

fn unique_name(label: &str) -> PackageName {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    PackageName::new(format!(
        "mgc-cache-{label}-{}-{sequence}",
        std::process::id()
    ))
    .unwrap()
}

#[test]
fn missing_package_returns_none() {
    let id = PackageId::new(unique_name("miss"), Version::new(1, 0, 0));

    assert!(MemoryCacheDaemon::global().get(&id).is_none());
}

#[test]
fn insert_and_get_share_the_cached_arc_bytes() {
    let id = PackageId::new(unique_name("roundtrip"), Version::new(1, 0, 0));
    let cache = MemoryCacheDaemon::global();
    let payload = b"cached package bytes";

    cache.insert(&id, payload);
    let first = cache.get(&id).expect("inserted entry must be present");
    let second = cache.get(&id).expect("entry must remain available");

    assert_eq!(first.as_ref(), payload);
    assert!(Arc::ptr_eq(&first, &second));
}

#[test]
fn cache_key_distinguishes_package_versions() {
    let name = unique_name("version");
    let first = PackageId::new(name.clone(), Version::new(1, 0, 0));
    let second = PackageId::new(name, Version::new(2, 0, 0));
    let cache = MemoryCacheDaemon::global();

    cache.insert(&first, b"version one");

    assert_eq!(cache.get(&first).unwrap().as_ref(), b"version one");
    assert!(cache.get(&second).is_none());
}
