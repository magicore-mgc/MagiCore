//! Destination preparation cases.
//! Ca chuẩn bị destination.

use super::super::prepare_destination_root;
use std::fs;

#[test]
fn creates_and_returns_a_canonical_missing_root() {
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("nested/backup");

    let canonical = prepare_destination_root(&destination).unwrap();

    assert!(canonical.is_dir());
    assert_eq!(fs::canonicalize(&destination).unwrap(), canonical);
}

#[cfg(unix)]
#[test]
fn rejects_final_symlink_destination_root() {
    use super::super::super::error::DestinationError;
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("link");
    fs::create_dir(&target).unwrap();
    symlink(&target, &link).unwrap();

    assert!(matches!(
        prepare_destination_root(&link),
        Err(DestinationError::SymlinkRoot(_))
    ));
}
