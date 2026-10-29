#![allow(clippy::unwrap_used)]

use super::*;

/// Every number comes from the filesystem: counts and bytes must match the
/// files a test creates.
/// (Mọi con số đến từ filesystem: số đếm và byte phải khớp file test tạo.)
#[test]
fn reports_counts_and_bytes_from_the_live_store() {
    let root = tempfile::tempdir().unwrap();
    let cas = root.path().join("cas").join("aa");
    std::fs::create_dir_all(&cas).unwrap();
    std::fs::write(cas.join("blob-one"), vec![0u8; 1024]).unwrap();
    std::fs::write(cas.join("blob-two"), vec![1u8; 512]).unwrap();

    let cache = root.path().join("cache");
    std::fs::create_dir_all(&cache).unwrap();
    std::fs::write(cache.join("pkg-1.0.0.tgz"), vec![2u8; 2048]).unwrap();
    // A non-tarball cache artefact is not a tarball entry and must not be
    // counted as one.
    // (Artifact không phải tarball trong cache không được tính là entry.)
    std::fs::write(cache.join("metadata.json"), b"{}").unwrap();

    let stats = collect(root.path()).unwrap();
    assert_eq!(stats.cas_blobs, 2);
    assert_eq!(stats.cas_bytes, 1536);
    assert_eq!(stats.cache_tarballs, 1);
    assert_eq!(stats.cache_bytes, 2048);
    assert_eq!(stats.payload_copies, 2);
    assert_eq!(stats.distinct_payloads, 2);
    assert_eq!(stats.dedupe_ratio, 0.0);
}

/// Hardlinks are one payload, so sharing shows up as a positive dedupe share.
/// (Hardlink là một payload nên chia sẻ hiện ra thành tỉ lệ dedupe dương.)
#[cfg(unix)]
#[test]
fn hardlinked_blobs_count_as_one_payload() {
    let root = tempfile::tempdir().unwrap();
    let cas = root.path().join("cas").join("bb");
    std::fs::create_dir_all(&cas).unwrap();
    let first = cas.join("blob-a");
    std::fs::write(&first, vec![7u8; 256]).unwrap();
    std::fs::hard_link(&first, cas.join("blob-b")).unwrap();

    let stats = collect(root.path()).unwrap();
    assert_eq!(stats.payload_copies, 2);
    assert_eq!(stats.distinct_payloads, 1);
    assert!((stats.dedupe_ratio - 0.5).abs() < f64::EPSILON);
}

/// A store that does not exist yet reports zeros instead of failing.
/// (Store chưa tồn tại báo 0 thay vì lỗi.)
#[test]
fn missing_store_reports_zeros() {
    let root = tempfile::tempdir().unwrap();
    let stats = collect(&root.path().join("absent")).unwrap();
    assert_eq!(stats.cas_blobs, 0);
    assert_eq!(stats.cas_bytes, 0);
    assert_eq!(stats.dedupe_ratio, 0.0);
}

/// The JSON object exposes exactly the fields a claim needs, so a script
/// cannot mistake a missing metric for a zero.
/// (Object JSON phơi đúng các trường claim cần, để script không nhầm thiếu
/// số đo thành 0.)
#[test]
fn json_shape_is_stable() {
    let root = tempfile::tempdir().unwrap();
    let stats = collect(root.path()).unwrap();
    let json = serde_json::to_value(&stats).unwrap();
    for key in [
        "root",
        "cas_blobs",
        "cas_bytes",
        "cache_tarballs",
        "cache_bytes",
        "memo_hits",
        "memo_misses",
        "memo_evictions",
        "distinct_payloads",
        "payload_copies",
        "dedupe_ratio",
    ] {
        assert!(json.get(key).is_some(), "missing field {key}");
    }
}
