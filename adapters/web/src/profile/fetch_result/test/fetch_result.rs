//! Tests for inline and cache-backed tarball payload contracts.
//! Kiểm tra hợp đồng payload tarball dạng bytes và cache-backed.

use super::{TarballFetchResult, TarballPayload};
use std::path::PathBuf;
use std::sync::Arc;

#[test]
fn payload_lengths_keep_bytes_and_cached_metadata_semantics() {
    let inline = TarballPayload::Bytes(Arc::<[u8]>::from(vec![1, 2, 3]));
    assert_eq!(inline.len(), 3);
    assert!(!inline.is_empty());

    let cached = TarballPayload::CachedPath(PathBuf::from("missing-cache-entry.tgz"), 41);
    assert_eq!(cached.len(), 41);
    assert!(!cached.is_empty());
    assert_eq!(
        TarballPayload::Bytes(Arc::<[u8]>::from(Vec::<u8>::new())).len(),
        0
    );

    let result = TarballFetchResult {
        payload: cached,
        queue_wait_ms: 7,
        io_ms: 11,
        persist_to_shared_cache: true,
    };
    assert_eq!(result.queue_wait_ms, 7);
    assert_eq!(result.io_ms, 11);
    assert!(result.persist_to_shared_cache);
}
