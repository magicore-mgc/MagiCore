//! A corrupt blob is moved aside, not served; an intact one is left alone.
//! Blob hỏng được chuyển đi, không phục vụ; blob nguyên vẹn giữ nguyên.

use super::super::{BlobState, verify_or_quarantine};
use crate::cas::integrity::IntegrityHash;

fn store(root: &std::path::Path, data: &[u8]) -> IntegrityHash {
    let hash = IntegrityHash::from_bytes(data, false);
    let path = hash.cas_path(root);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, data).unwrap();
    hash
}

#[test]
fn an_absent_blob_is_reported_missing() {
    let dir = tempfile::tempdir().unwrap();
    let hash = IntegrityHash::from_bytes(b"never written", false);
    assert_eq!(
        verify_or_quarantine(dir.path(), &hash).unwrap(),
        BlobState::Missing
    );
}

#[test]
fn an_intact_blob_is_left_in_place() {
    let dir = tempfile::tempdir().unwrap();
    let hash = store(dir.path(), b"correct bytes");
    assert_eq!(
        verify_or_quarantine(dir.path(), &hash).unwrap(),
        BlobState::Intact
    );
    assert!(hash.cas_path(dir.path()).is_file());
}

#[test]
fn a_corrupt_blob_is_quarantined_and_no_longer_served() {
    // The address is only a name: bytes written under it can change, and a
    // store that serves them anyway returns the wrong artefact.
    // Địa chỉ chỉ là cái tên: byte ghi dưới nó có thể đổi, và store phục vụ
    // chúng sẽ trả artefact sai.
    let dir = tempfile::tempdir().unwrap();
    let hash = store(dir.path(), b"correct bytes");
    let path = hash.cas_path(dir.path());
    std::fs::write(&path, b"tampered bytes!").unwrap();

    let state = verify_or_quarantine(dir.path(), &hash).unwrap();
    let BlobState::Quarantined { moved_to } = state else {
        panic!("a corrupt blob must be quarantined, got {state:?}");
    };
    assert!(moved_to.is_file(), "the blob is kept as evidence");
    assert!(!path.exists(), "the corrupt blob is no longer in place");
    assert!(
        moved_to.to_string_lossy().ends_with(".corrupt"),
        "the quarantine name records why: {}",
        moved_to.display()
    );
}
