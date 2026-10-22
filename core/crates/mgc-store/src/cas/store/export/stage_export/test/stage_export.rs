//! Stage Export source regression test.
//! Test hồi quy source cho owner stage export.

use std::fs::{self, File};

use crate::cas::store::StagingKind;

#[test]
fn staging_copies_bytes_and_reports_the_selected_strategy() {
    let temp = tempfile::tempdir().unwrap();
    let src = temp.path().join("source");
    let dest = temp.path().join("stage");
    fs::write(&src, b"payload").unwrap();

    let source = File::open(&src).unwrap();
    let strategy = super::super::stage_export::stage_export_bytes(&source, &dest).unwrap();

    assert!(matches!(
        strategy,
        StagingKind::CowClone | StagingKind::PlainCopy
    ));
    assert_eq!(fs::read(&dest).unwrap(), b"payload");
}

#[cfg(unix)]
#[test]
fn staging_keeps_reading_the_opened_object_after_path_replacement() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let source_path = temp.path().join("source");
    let moved_path = temp.path().join("moved-source");
    let replacement_path = temp.path().join("replacement");
    let destination = temp.path().join("stage");
    fs::write(&source_path, b"opened source").unwrap();
    fs::write(&replacement_path, b"replacement source").unwrap();

    let source = File::open(&source_path).unwrap();
    fs::rename(&source_path, &moved_path).unwrap();
    symlink(&replacement_path, &source_path).unwrap();

    super::super::stage_export::stage_export_bytes(&source, &destination).unwrap();

    assert_eq!(fs::read(&destination).unwrap(), b"opened source");
}
