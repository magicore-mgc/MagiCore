//! Verifies stage errors keep their I/O source and export boundary mapping.
//! Kiểm tra lỗi stage giữ source I/O và mapping tại ranh giới export.

use std::error::Error;
use std::fs::{self, File};

use crate::cas::store::StoreError;

#[test]
fn existing_destination_preserves_the_copy_error_and_path() {
    let temp = tempfile::tempdir().unwrap();
    let source_path = temp.path().join("source");
    let destination = temp.path().join("occupied");
    fs::write(&source_path, b"payload").unwrap();
    fs::create_dir(&destination).unwrap();
    let source = File::open(source_path).unwrap();

    let error = super::super::stage_export::stage_export_bytes(&source, &destination).unwrap_err();

    assert!(Error::source(&error).is_some());
    match error.into_store_error() {
        StoreError::Io { path, msg } => {
            assert_eq!(path, destination);
            assert!(msg.starts_with("copy blob for export failed: "));
        }
        other => panic!("expected an I/O error, received {other:?}"),
    }
}
