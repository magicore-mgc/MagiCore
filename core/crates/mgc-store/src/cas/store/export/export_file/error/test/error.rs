//! Tests stable CAS export source errors for missing and non-regular paths.
//! Kiểm thử lỗi nguồn ổn định của CAS export cho path thiếu/không thường.

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::export::export_file::error::{ExportSourceError, open_cas_blob_source};

#[test]
fn missing_blob_keeps_the_content_not_found_category() {
    let temp = tempfile::tempdir().unwrap();
    let hash = IntegrityHash::from_bytes(b"missing", false);

    let error = open_cas_blob_source(&temp.path().join("missing.blob"), &hash).unwrap_err();

    assert!(matches!(error, ExportSourceError::NotFound { digest } if digest == hash.as_hex()));
}

#[test]
fn directory_keeps_the_export_specific_diagnostic() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("directory");
    std::fs::create_dir(&path).unwrap();
    let hash = IntegrityHash::from_bytes(b"directory", false);

    let error = open_cas_blob_source(&path, &hash).unwrap_err();

    assert!(matches!(error, ExportSourceError::NotRegular));
    assert_eq!(error.to_string(), "CAS blob path is not a regular file");
}

#[cfg(unix)]
#[test]
fn final_symlink_is_rejected_with_the_export_diagnostic() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("blob");
    std::fs::write(&target, b"target").unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();
    let hash = IntegrityHash::from_bytes(b"target", false);

    let error = open_cas_blob_source(&link, &hash).unwrap_err();

    assert!(matches!(error, ExportSourceError::NotRegular));
    assert_eq!(error.to_string(), "CAS blob path is not a regular file");
}
