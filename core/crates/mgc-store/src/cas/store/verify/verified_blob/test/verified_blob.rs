//! Tests memo hits and quarantine after stored-blob corruption.
//! Kiểm thử memo hit và cách ly blob đã lưu bị sửa.

use crate::cas::store::{ContentStore, StoreError};

#[cfg(unix)]
#[test]
fn memo_reuses_verification_when_file_identity_is_unchanged() {
    use std::num::NonZeroUsize;

    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas"))
        .unwrap()
        .with_memo_capacity(NonZeroUsize::new(2).unwrap());
    let hash = store.import_bytes(b"verified blob").unwrap();

    store.verify_stored_blob(&hash).unwrap();
    store.verify_stored_blob(&hash).unwrap();

    let stats = store.memo_stats();
    assert_eq!(stats.misses, 1);
    assert_eq!(stats.hits, 1);
}

#[test]
fn mismatched_stored_blob_is_rejected_and_copied_to_quarantine() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"original blob").unwrap();
    std::fs::write(hash.cas_path(&store.root), b"modified blob").unwrap();

    let error = store.verify_stored_blob(&hash).unwrap_err();

    assert!(matches!(error, StoreError::HashMismatch { .. }));
    assert_eq!(
        std::fs::read(hash.cas_path(&store.root)).unwrap(),
        b"modified blob"
    );
    let quarantine = store.root.join("quarantine");
    assert!(std::fs::read_dir(quarantine).unwrap().next().is_some());
}

#[test]
fn quarantine_io_failure_does_not_mask_the_hash_mismatch() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"original blob").unwrap();
    std::fs::write(hash.cas_path(&store.root), b"modified blob").unwrap();
    std::fs::write(store.root.join("quarantine"), b"not a directory").unwrap();

    let error = store.verify_stored_blob(&hash).unwrap_err();

    assert!(matches!(error, StoreError::HashMismatch { .. }));
    assert_eq!(
        std::fs::read(hash.cas_path(&store.root)).unwrap(),
        b"modified blob"
    );
}
