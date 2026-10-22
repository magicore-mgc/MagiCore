//! Symlink entry tests — candidates are preserved and nested links are not counted.
//! Test symlink entry — giữ package link và không cộng payload link lồng bên trong.
use super::super::{cleanup_unreferenced_vstore, vstore_root};
use super::symlink_support::{symlink_directory, symlink_file};
use mgc_lockfile::Lockfile;
use std::fs;

#[cfg(any(unix, windows))]
#[test]
fn cleanup_preserves_symlinked_package_candidate() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let external_package = temp.path().join("external-package");
    let candidate = vstore_root(&project).join("orphan@2.0.0");
    fs::create_dir_all(candidate.parent().unwrap()).unwrap();
    fs::create_dir_all(&external_package).unwrap();
    fs::write(external_package.join("payload.bin"), b"outside").unwrap();
    symlink_directory(&external_package, &candidate).unwrap();

    let report = cleanup_unreferenced_vstore(&project, &Lockfile::new());

    assert_eq!(report.deleted_count(), 0);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 0);
    assert_eq!(report.failed_count(), 1);
    assert!(
        fs::symlink_metadata(&candidate)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(external_package.join("payload.bin").is_file());
}

#[cfg(any(unix, windows))]
#[test]
fn cleanup_ignores_nested_file_and_directory_symlinks_when_measuring() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    let candidate = vstore_root(&project).join("orphan@2.0.0");
    let external_file = temp.path().join("external.bin");
    let external_directory = temp.path().join("external-directory");
    fs::create_dir_all(&candidate).unwrap();
    fs::create_dir_all(&external_directory).unwrap();
    fs::write(candidate.join("payload.bin"), b"four").unwrap();
    fs::write(&external_file, b"secret").unwrap();
    fs::write(external_directory.join("payload.bin"), b"outside").unwrap();
    symlink_file(&external_file, &candidate.join("external-file-link")).unwrap();
    symlink_directory(&external_directory, &candidate.join("external-dir-link")).unwrap();

    let report = cleanup_unreferenced_vstore(&project, &Lockfile::new());

    assert_eq!(report.deleted_count(), 1);
    assert_eq!(report.logical_payload_bytes_in_removed_paths(), 4);
    assert_eq!(report.failed_count(), 0);
    assert!(external_file.is_file());
    assert!(external_directory.join("payload.bin").is_file());
}
