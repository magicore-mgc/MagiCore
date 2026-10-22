//! Atomic publication tests / Test publish nguyên tử.
#![allow(clippy::unwrap_used)]

use crate::install::lockfile::test::fixtures::package;
use crate::install::lockfile::{atomic_write_canonical_lock, write_canonical_lock};
use mgc_lockfile::EcosystemTag;

#[cfg(unix)]
#[test]
fn lock_writer_refuses_symlink_without_touching_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("target.lock");
    let link = dir.path().join("mgc.lock");
    let original = b"untrusted target bytes";
    std::fs::write(&target, original).unwrap();
    symlink(&target, &link).unwrap();

    let result = write_canonical_lock(
        dir.path(),
        &[EcosystemTag::Dart],
        vec![package("flutter-dep", "1.0.0", EcosystemTag::Dart)],
    );

    assert!(result.is_err());
    assert_eq!(std::fs::read(target).unwrap(), original);
    assert!(
        std::fs::symlink_metadata(link)
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn atomic_lock_publish_replaces_existing_file_without_leaking_temp() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    std::fs::write(
        &path,
        concat!(
            "version = \"3\"\n[metadata]\n",
            "generated_at = \"1970-01-01T00:00:00Z\"\n",
            "generator = \"fixture\"\nlockfile_hash = \"\"\n",
        )
        .as_bytes(),
    )
    .unwrap();

    let replacement = concat!(
        "version = \"3\"\n[metadata]\n",
        "generated_at = \"1970-01-01T00:00:01Z\"\n",
        "generator = \"fixture\"\nlockfile_hash = \"\"\n",
    )
    .as_bytes();
    atomic_write_canonical_lock(&path, replacement).unwrap();

    assert_eq!(std::fs::read(path).unwrap(), replacement);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}
