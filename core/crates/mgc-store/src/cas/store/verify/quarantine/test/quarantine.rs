//! Tests handle-bound evidence copies and reports.
//! Kiểm thử bản sao bằng chứng và báo cáo gắn với file handle.

use super::super::super::QuarantineError;
use super::super::quarantine_blob;

#[test]
fn copies_opened_blob_and_retains_source() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("blob");
    std::fs::write(&source, b"corrupt").unwrap();
    let opened = std::fs::File::open(&source).unwrap();
    let expected = "a".repeat(64);
    let actual = blake3::hash(b"corrupt").to_hex().to_string();

    quarantine_blob(&root, &source, opened, &expected, &actual).unwrap();

    assert_eq!(std::fs::read(&source).unwrap(), b"corrupt");
    let files = std::fs::read_dir(root.join("quarantine")).unwrap();
    let paths = files.map(|entry| entry.unwrap().path()).collect::<Vec<_>>();
    let blob = paths
        .iter()
        .find(|path| path.extension().is_some_and(|ext| ext == "blob"))
        .unwrap();
    assert_eq!(std::fs::read(blob).unwrap(), b"corrupt");
    let report = paths
        .iter()
        .find(|path| path.extension().is_some_and(|ext| ext == "report"))
        .unwrap();
    let text = std::fs::read_to_string(report).unwrap();
    assert!(text.contains(&format!("expected blake3: {expected}")));
    assert!(text.contains(&format!("actual blake3:   {actual}")));
    assert!(text.contains("evidence captured from verified handle: true"));
    assert!(text.contains("source retained: true"));
}

#[test]
fn path_replacement_does_not_redirect_evidence_or_get_moved() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("blob");
    let displaced = root.join("displaced");
    let replacement = root.join("replacement");
    std::fs::write(&source, b"opened bytes").unwrap();
    let opened = std::fs::File::open(&source).unwrap();
    std::fs::rename(&source, &displaced).unwrap();
    std::fs::write(&replacement, b"replacement bytes").unwrap();
    std::fs::rename(&replacement, &source).unwrap();

    let actual = blake3::hash(b"opened bytes").to_hex().to_string();
    quarantine_blob(&root, &source, opened, &"a".repeat(64), &actual).unwrap();

    assert_eq!(std::fs::read(&source).unwrap(), b"replacement bytes");
    let files = std::fs::read_dir(root.join("quarantine")).unwrap();
    let blob = files
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|ext| ext == "blob"))
        .unwrap();
    assert_eq!(std::fs::read(blob).unwrap(), b"opened bytes");
}

#[test]
fn mismatched_evidence_digest_is_discarded_with_a_typed_error() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("blob");
    std::fs::write(&source, b"verified bytes").unwrap();
    let opened = std::fs::File::open(&source).unwrap();
    let wrong_actual = blake3::hash(b"different bytes").to_hex().to_string();

    let result = quarantine_blob(&root, &source, opened, &"a".repeat(64), &wrong_actual);

    assert!(matches!(
        result,
        Err(QuarantineError::DigestMismatch { .. })
    ));
    assert_eq!(std::fs::read(&source).unwrap(), b"verified bytes");
    assert!(
        std::fs::read_dir(root.join("quarantine"))
            .unwrap()
            .next()
            .is_none()
    );
}
