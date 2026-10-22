use super::super::{SafeFilesystemError, logical_payload_bytes};
#[cfg(any(unix, windows))]
use super::symlink_support::{symlink_directory, symlink_file};

#[cfg(any(unix, windows))]
#[test]
fn logical_payload_counts_regular_files_and_skips_symlinks() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("root");
    let nested = root.join("nested");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(nested.join("payload.bin"), [0u8; 7]).unwrap();
    let external = temp.path().join("external.bin");
    std::fs::write(&external, [0u8; 4096]).unwrap();
    symlink_file(&external, &nested.join("external-link")).unwrap();

    assert_eq!(logical_payload_bytes(&root).unwrap(), 7);
}

#[cfg(any(unix, windows))]
#[test]
fn logical_payload_rejects_a_symlink_root() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    std::fs::create_dir(&target).unwrap();
    let root = temp.path().join("link");
    symlink_directory(&target, &root).unwrap();

    assert!(matches!(
        logical_payload_bytes(&root),
        Err(SafeFilesystemError::SymlinkedDirectory { .. })
    ));
}
