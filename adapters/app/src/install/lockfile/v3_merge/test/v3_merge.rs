//! v3 lane merge tests / Test gộp lane v3.
#![allow(clippy::unwrap_used)]

use crate::install::lockfile::test::fixtures::{package, v4_package, write_mgc_toml_schema};
use crate::install::lockfile::write_canonical_lock;
use mgc_lockfile::{EcosystemTag, Lockfile};

#[test]
fn app_writer_refuses_signed_lock_and_preserves_signature_pair() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let mut signed = Lockfile::new();
    signed
        .packages
        .push(package("old-app-dep", "1.0.0", EcosystemTag::Dart));
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut signed, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();

    let result = write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("new-app-dep", "2.0.0", EcosystemTag::Dart)],
    );

    assert!(result.unwrap_err().to_string().contains("signed mgc.lock"));
    assert_eq!(std::fs::read(&path).unwrap(), lock_before);
    assert_eq!(std::fs::read(&signature_path).unwrap(), signature_before);
    assert_eq!(
        mgc_lockfile::verify_lockfile(&path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
}

#[test]
fn app_writer_leaves_an_unchanged_signed_lock_byte_identical() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "app").unwrap();
    let path = dir.path().join("mgc.lock");
    let mut existing = Lockfile::new();
    let mut existing_package = package("stable-app-dep", "1.0.0", EcosystemTag::Dart);
    existing_package.owner_core = Some("app".to_string());
    existing.packages.push(existing_package);
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut existing, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();
    let mut next_package = package("stable-app-dep", "1.0.0", EcosystemTag::Dart);
    next_package.owner_core = Some("app".to_string());

    write_canonical_lock(dir.path(), &[EcosystemTag::Dart], vec![next_package]).unwrap();

    assert_eq!(std::fs::read(&path).unwrap(), lock_before);
    assert_eq!(std::fs::read(&signature_path).unwrap(), signature_before);
    assert_eq!(
        mgc_lockfile::verify_lockfile(&path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
}

#[test]
fn v3_schema_keeps_v3_lock() {
    let dir = tempfile::tempdir().unwrap();
    write_mgc_toml_schema(dir.path(), "v3");
    write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![v4_package("http", "1.0.0")],
    )
    .unwrap();
    let text = std::fs::read_to_string(dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        mgc_lockfile::migrate::detect_lockfile_version(&text).unwrap(),
        3
    );
}

#[test]
fn v3_schema_meets_v4_lock_with_actionable_error() {
    // Schema v3 (default) facing a v4 lock fails with guidance, not a bare
    // parse error: point at `[lock] schema = "v4"` or a v3 re-install.
    // (Schema v3 (mặc định) gặp lock v4 thì fail kèm hướng dẫn, không phải
    // lỗi parse trần.)
    let dir = tempfile::tempdir().unwrap();
    write_mgc_toml_schema(dir.path(), "v3");
    std::fs::write(
        dir.path().join("mgc.lock"),
        "version = \"4\"\n[metadata]\ngenerated_at = \"x\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n",
    )
    .unwrap();
    let err = write_canonical_lock(dir.path(), &[EcosystemTag::Dart], vec![]).unwrap_err();
    assert!(
        err.to_string().contains("[lock] schema"),
        "expected actionable schema error, got: {err}"
    );
}
