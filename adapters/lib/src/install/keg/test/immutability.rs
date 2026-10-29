//! Immutability tests: seal, verify, tamper detection, thaw, remove.
//! Test bất biến: seal, verify, phát hiện sửa, rã băng, xoá.

use super::super::error::KegError;
use super::super::immutability::{remove_keg, seal_keg, thaw_keg, verify_keg};
use super::super::keg::KegStore;
use super::support::{Fixture, mode_of, thaw_all, write_tree};

fn staging_tree(fixture: &Fixture, name: &str, version: &str) -> std::path::PathBuf {
    let staging = fixture.layout.cellar_root().join("staging-work");
    std::fs::create_dir_all(&staging).unwrap();
    write_tree(&staging, name, version).unwrap();
    staging
}

#[test]
fn sealing_removes_every_write_bit_and_verify_accepts_it() {
    let fixture = Fixture::new();
    let staging = staging_tree(&fixture, "libcxx", "1.0.0");

    let fingerprint = seal_keg(&staging).unwrap();

    assert_eq!(mode_of(&staging) & 0o222, 0);
    assert_eq!(mode_of(&staging.join("README")) & 0o222, 0);
    assert!(fingerprint.file_count >= 3);
    verify_keg(&staging, &fingerprint).unwrap();
    thaw_all(&fixture, "libcxx");
}

#[test]
fn verify_detects_content_change_even_after_chmod() {
    let fixture = Fixture::new();
    let staging = staging_tree(&fixture, "libcxx", "1.0.0");
    let fingerprint = seal_keg(&staging).unwrap();

    // Attacker restores the write bit (chmod / root) and edits a file.
    // (Kẻ tấn công khôi phục bit write rồi sửa file.)
    let target = staging.join("README");
    let mut permissions = std::fs::metadata(&target).unwrap().permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.set_mode(0o644);
        std::fs::set_permissions(&target, permissions).unwrap();
    }
    std::fs::write(&target, b"tampered").unwrap();

    let error = verify_keg(&staging, &fingerprint).expect_err("tampering must be detected");
    assert!(matches!(error, KegError::Tampered { .. }), "{error:?}");
    thaw_all(&fixture, "libcxx");
}

#[test]
fn verify_detects_a_restored_write_bit_without_content_change() {
    let fixture = Fixture::new();
    let staging = staging_tree(&fixture, "libcxx", "1.0.0");
    let fingerprint = seal_keg(&staging).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            staging.join("README"),
            std::fs::Permissions::from_mode(0o644),
        )
        .unwrap();
        let error =
            verify_keg(&staging, &fingerprint).expect_err("a writable keg is no longer immutable");
        assert!(matches!(error, KegError::Tampered { .. }), "{error:?}");
    }
    thaw_all(&fixture, "libcxx");
}

#[test]
fn verify_detects_an_absolute_or_escaping_symlink() {
    let fixture = Fixture::new();
    let staging = staging_tree(&fixture, "libcxx", "1.0.0");
    let fingerprint = seal_keg(&staging).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let link = staging.join("escape");
        std::fs::set_permissions(&staging, std::fs::Permissions::from_mode(0o755)).unwrap();
        std::os::unix::fs::symlink("/etc/passwd", &link).unwrap();
        let error = verify_keg(&staging, &fingerprint).expect_err("absolute link must be rejected");
        assert!(matches!(error, KegError::UnsafePath { .. }), "{error:?}");
        std::fs::remove_file(&link).unwrap();
    }
    thaw_all(&fixture, "libcxx");
}

#[test]
fn thaw_restores_owner_write_and_remove_deletes_the_tree() {
    let fixture = Fixture::new();
    fixture.install("libcxx", "1.0.0");
    let keg = fixture.layout.keg_dir(
        "libcxx",
        &super::super::layout::validate_version("1.0.0").unwrap(),
    );

    let restored = thaw_keg(&keg).unwrap();
    assert!(restored > 0, "sealed keg must have entries to thaw");
    assert_ne!(
        mode_of(&keg) & 0o200,
        0,
        "keg root must be writable for removal"
    );

    remove_keg(&keg).unwrap();
    assert!(!keg.exists());
}

#[test]
fn removing_a_missing_keg_is_a_noop() {
    let fixture = Fixture::new();
    let missing = fixture.layout.keg_dir(
        "libcxx",
        &super::super::layout::validate_version("0.0.1").unwrap(),
    );
    remove_keg(&missing).unwrap();
}

#[test]
fn tampered_keg_is_refused_by_open_keg() {
    let fixture = Fixture::new();
    let keg = fixture.install("libcxx", "1.0.0");
    let _store = KegStore::new(fixture.layout.clone());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let readable = keg.join("README");
        std::fs::set_permissions(keg.clone(), std::fs::Permissions::from_mode(0o755)).unwrap();
        std::fs::set_permissions(&readable, std::fs::Permissions::from_mode(0o644)).unwrap();
        std::fs::write(&readable, b"swapped").unwrap();
        let error = fixture.store.open_keg("libcxx", "1.0.0").unwrap_err();
        assert!(matches!(error, KegError::Tampered { .. }), "{error:?}");
    }
    thaw_all(&fixture, "libcxx");
}
