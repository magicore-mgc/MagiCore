//! Prune policy cases — xác nhận retention, tie-break và dữ liệu thiếu.

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::policy::select_removals;
use crate::cache::metadata::CacheEntry;
use crate::cache::prune::candidate_discovery::CacheCandidate;
use crate::cache::prune::types::PruneStrategy;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn candidate(name: &str, last_accessed: Option<SystemTime>, content_bytes: u64) -> CacheCandidate {
    CacheCandidate {
        path: PathBuf::from(format!("/cache/{name}")),
        cache_model_id: name.into(),
        metadata: last_accessed.map(|last_accessed| CacheEntry {
            model_id: name.into(),
            size_bytes: content_bytes,
            cached_at: SystemTime::now(),
            last_accessed,
            checksum: None,
        }),
        metadata_sidecar: None,
        content_bytes: Some(content_bytes),
    }
}

#[test]
fn keep_recent_retains_newest_and_breaks_ties_by_path() {
    let now = SystemTime::now();
    let candidates = vec![
        candidate("a", Some(now), 1),
        candidate("b", Some(now), 1),
        candidate("c", Some(now - Duration::from_secs(1)), 1),
    ];

    assert_eq!(
        select_removals(&candidates, PruneStrategy::KeepRecent(1)),
        vec![1, 2]
    );
}

#[test]
fn keep_recent_zero_removes_known_entries_but_retains_unknown_metadata() {
    let candidates = vec![
        candidate("known", Some(SystemTime::now()), 1),
        candidate("unknown", None, 1),
    ];

    assert_eq!(
        select_removals(&candidates, PruneStrategy::KeepRecent(0)),
        vec![0]
    );
}

#[test]
fn larger_than_uses_measured_content_size() {
    let candidates = vec![candidate("small", Some(SystemTime::now()), 3)];

    assert!(select_removals(&candidates, PruneStrategy::LargerThan(2)).contains(&0));
    assert!(select_removals(&candidates, PruneStrategy::LargerThan(3)).is_empty());
}
