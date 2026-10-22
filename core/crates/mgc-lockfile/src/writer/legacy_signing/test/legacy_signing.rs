//! Tests legacy signing and schema rejection.
//! Kiểm tra ký legacy và từ chối sai schema.

use super::super::sign_and_write_lockfile;
use crate::{EcosystemTag, Lockfile, LockfileError, Package, VerificationStatus};

#[test]
fn signing_writes_verifiable_sidecar_for_legacy_lock() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    let mut lock = Lockfile::new();

    sign_and_write_lockfile(
        &mut lock,
        &lock_path,
        &mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed"),
    )
    .expect("test operation must succeed");

    assert_eq!(
        crate::verify_lockfile(&lock_path).expect("test operation must succeed"),
        VerificationStatus::Valid
    );
}

#[test]
fn legacy_signer_refuses_v4_document() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    let mut document = crate::canonical::LockfileV4::new("mgc/test");
    let bytes =
        crate::writer::finalize_v4_document(&mut document).expect("test operation must succeed");
    std::fs::write(&lock_path, bytes).expect("test operation must succeed");

    assert!(matches!(
        sign_and_write_lockfile(
            &mut Lockfile::new(),
            &lock_path,
            &mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed"),
        ),
        Err(LockfileError::WriteFailed(message)) if message.contains("cannot mutate v4")
    ));
}

#[test]
fn incomplete_source_is_rejected_before_signing_mutates_or_writes() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    let mut lock = Lockfile::new();
    let mut package = Package::new("serde".into(), "1.0.0".into(), "".into(), "".into());
    package.ecosystem = EcosystemTag::Rust;
    lock.add_package(package);

    let result = sign_and_write_lockfile(
        &mut lock,
        &lock_path,
        &mgc_crypto::keyring::KeyPair::generate().expect("test operation must succeed"),
    );

    assert!(matches!(
        result,
        Err(LockfileError::IncompleteProvenance(_))
    ));
    assert!(lock.metadata.signer.is_none());
    assert!(lock.metadata.lockfile_hash.is_empty());
    assert!(!lock_path.exists());
    assert!(!lock_path.with_extension("lock.sig").exists());
}
