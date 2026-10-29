//! Process-local cache behavior tests. / Test hành vi cache theo process.

#![allow(clippy::unwrap_used)]

use crate::cache_daemon::MemoryCacheDaemon;
use mgc_types::{PackageId, PackageName, Version};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Build a package id whose name is stable, so re-inserting targets one key.
/// (Package id có tên ổn định để chèn lại đúng một key.)
fn fixed_package(name: &str) -> PackageId {
    PackageId::new(
        PackageName::new(name.to_string()).expect("valid name"),
        Version::new(1, 0, 0),
    )
}

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

// ── Bounded eviction — eviction có giới hạn ──────────────────────────────

/// A bounded daemon evicts only a quarter, not the whole map.
///
/// Clearing everything on overflow (the previous behaviour) discarded entries
/// the warm install was about to read.
/// (Daemon có giới hạn chỉ evict 1/4, không xoá sạch — hành vi cũ làm mất
/// entry lượt cài ấm sắp đọc.)
#[test]
fn overflow_evicts_a_quarter_and_keeps_the_newest() {
    let daemon = MemoryCacheDaemon::with_capacity(8);
    let ids: Vec<PackageId> = (0..8)
        .map(|i| fixed_package(&format!("mgc-cache-evict-{i}")))
        .collect();
    for id in &ids {
        daemon.insert(id, b"x");
    }
    assert_eq!(daemon.len(), 8);

    let newest = fixed_package("mgc-cache-evict-newest");
    daemon.insert(&newest, b"x");
    // 9 entries exceeds 8 → two oldest dropped, newest retained.
    assert!(daemon.len() < 9, "map must shrink after overflow");
    assert!(
        daemon.get(&newest).is_some(),
        "the entry that triggered eviction must survive"
    );
    assert!(
        daemon.get(&ids[0]).is_none(),
        "the oldest entry is evicted first"
    );
}

/// The daemon never grows past its bound no matter how many inserts happen.
/// (Daemon không bao giờ vượt giới hạn dù chèn bao nhiêu lần.)
#[test]
fn repeated_inserts_stay_within_the_bound() {
    let daemon = MemoryCacheDaemon::with_capacity(16);
    for index in 0..500 {
        daemon.insert(&fixed_package(&format!("mgc-cache-bound-{index}")), b"x");
        assert!(
            daemon.len() <= 17,
            "len {} exceeded the bound after insert {index}",
            daemon.len()
        );
    }
}

/// A single-entry bound still works: inserting a second key evicts the first.
/// (Giới hạn 1 entry vẫn chạy: chèn key thứ hai thì evict key thứ nhất.)
#[test]
fn a_single_entry_bound_keeps_one_entry() {
    let daemon = MemoryCacheDaemon::with_capacity(1);
    let first = fixed_package("mgc-cache-one-first");
    let second = fixed_package("mgc-cache-one-second");
    daemon.insert(&first, b"a");
    daemon.insert(&second, b"b");
    assert_eq!(daemon.len(), 1);
    assert!(daemon.get(&second).is_some());
    assert!(daemon.get(&first).is_none());
}

/// The same package id replaces its own entry instead of duplicating it.
/// (Cùng package id thì thay entry của chính nó, không nhân đôi.)
#[test]
fn reinserting_the_same_package_does_not_grow_the_map() {
    let daemon = MemoryCacheDaemon::with_capacity(4);
    let id = fixed_package("mgc-cache-reinsert-probe");
    daemon.insert(&id, b"first");
    daemon.insert(&id, b"second");
    assert_eq!(daemon.len(), 1);
    assert_eq!(&*daemon.get(&id).expect("entry"), &b"second"[..]);
}

/// A capacity of zero is clamped to one so the cache cannot livelock on eviction.
/// (Giới hạn 0 được kẹp về 1 để cache không lặp vô hạn khi evict.)
#[test]
fn zero_capacity_is_clamped_to_one() {
    let daemon = MemoryCacheDaemon::with_capacity(0);
    daemon.insert(&fixed_package("mgc-cache-zero-only"), b"x");
    assert_eq!(daemon.len(), 1);
}

/// An empty daemon reports empty and returns nothing for any id.
/// (Daemon rỗng báo rỗng và không trả gì cho mọi id.)
#[test]
fn an_empty_daemon_holds_nothing() {
    let daemon = MemoryCacheDaemon::with_capacity(4);
    assert!(daemon.is_empty());
    assert!(daemon.get(&fixed_package("mgc-cache-absent")).is_none());
}
