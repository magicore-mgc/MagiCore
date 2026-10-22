#![cfg(test)]

use mgc_lockfile::{LockDocument, canonical::LockfileV4, parser::parse_document};
use std::io::Write;

#[test]
fn document_dispatch_preserves_v4_identity_and_reads_legacy_versions() {
    let mut v4 = LockfileV4::new("mgc/1.2.0");
    v4.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&v4.payload());
    let text =
        mgc_lockfile::canonical::write_v4_document(&v4).expect("test operation must succeed");

    let parsed = parse_document(&text).expect("test operation must succeed");
    assert!(matches!(parsed, LockDocument::V4(_)));

    let legacy = "version = \"3\"\npackage = []\n\n[metadata]\ngenerated_at = \"now\"\ngenerator = \"mgc\"\nlockfile_hash = \"\"\n";
    assert!(matches!(
        parse_document(legacy).expect("test operation must succeed"),
        LockDocument::Legacy(_)
    ));
}

#[test]
fn document_dispatch_rejects_unknown_schema_versions() {
    let unknown = "version = \"99\"\n";
    assert!(parse_document(unknown).is_err());
}

#[test]
fn file_loader_preserves_v4_and_legacy_loader_refuses_lossy_read() {
    let mut v4 = LockfileV4::new("mgc/1.2.0");
    v4.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&v4.payload());
    let bytes =
        mgc_lockfile::canonical::write_v4_document(&v4).expect("test operation must succeed");
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let path = dir.path().join("mgc.lock");
    std::fs::File::create(&path)
        .expect("test operation must succeed")
        .write_all(bytes.as_bytes())
        .expect("test operation must succeed");

    let loaded = mgc_lockfile::load_lock_document(&path).expect("test operation must succeed");
    assert!(matches!(loaded, LockDocument::V4(_)));

    let error = mgc_lockfile::load_lockfile(&path)
        .expect_err("test operation must fail")
        .to_string();
    assert!(error.contains("v4"), "unexpected error: {error}");
    assert!(error.contains("lossless"), "unexpected error: {error}");
}

#[test]
fn file_loader_dispatches_legacy_lockfiles() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let path = dir.path().join("mgc.lock");
    std::fs::write(
        &path,
        "version = \"3\"\npackage = []\n\n[metadata]\ngenerated_at = \"now\"\ngenerator = \"mgc\"\nlockfile_hash = \"\"\n",
    )
    .expect("test operation must succeed");

    assert!(matches!(
        mgc_lockfile::load_lock_document(&path).expect("test operation must succeed"),
        LockDocument::Legacy(_)
    ));
}

#[test]
fn legacy_sidecar_verifier_refuses_inline_v4_signatures() {
    let mut v4 = LockfileV4::new("mgc/1.2.0");
    v4.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&v4.payload());
    let bytes =
        mgc_lockfile::canonical::write_v4_document(&v4).expect("test operation must succeed");
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");
    let signature_path = dir.path().join("mgc.lock.sig");
    std::fs::write(&lock_path, bytes).expect("test operation must succeed");

    let error = mgc_lockfile::load_and_verify_lockfile(&lock_path, &signature_path)
        .expect_err("test operation must fail")
        .to_string();
    assert!(error.contains("v4"), "unexpected error: {error}");
    assert!(error.contains("lossless"), "unexpected error: {error}");
}

#[test]
fn shared_verifier_dispatches_unsigned_v4_without_flattening() {
    let mut v4 = LockfileV4::new("mgc/1.2.0");
    v4.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&v4.payload());
    let bytes =
        mgc_lockfile::canonical::write_v4_document(&v4).expect("test operation must succeed");
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");
    std::fs::write(&lock_path, bytes).expect("test operation must succeed");

    assert_eq!(
        mgc_lockfile::verify_lockfile(&lock_path).expect("test operation must succeed"),
        mgc_lockfile::VerificationStatus::Unsigned
    );
}
