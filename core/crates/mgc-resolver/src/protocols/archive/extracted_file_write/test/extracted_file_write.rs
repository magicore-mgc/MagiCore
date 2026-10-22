//! Tests atomic file replacement and cleanup after publish failure.
//! Kiểm thử thay file nguyên tử và dọn staging khi publish thất bại.

use super::super::*;
#[test]
fn extracted_files_replace_regular_files_without_leaking_temp_files() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let dest = root.path().join("file.txt");
    std::fs::write(&dest, b"old").expect("test operation must succeed");

    write_extracted_file(&dest, b"new").expect("test operation must succeed");

    assert_eq!(
        std::fs::read(&dest).expect("test operation must succeed"),
        b"new"
    );
    assert_eq!(
        std::fs::read_dir(root.path())
            .expect("test operation must succeed")
            .count(),
        1,
        "successful publish leaves no extraction temp"
    );
}

#[test]
fn failed_extracted_file_publish_preserves_destination_and_cleans_temp() {
    let root = tempfile::tempdir().expect("test operation must succeed");
    let dest = root.path().join("existing-dir");
    std::fs::create_dir(&dest).expect("test operation must succeed");
    std::fs::write(dest.join("keep.txt"), b"original").expect("test operation must succeed");

    assert!(write_extracted_file(&dest, b"replacement").is_err());

    assert_eq!(
        std::fs::read(dest.join("keep.txt")).expect("test operation must succeed"),
        b"original"
    );
    assert_eq!(
        std::fs::read_dir(root.path())
            .expect("test operation must succeed")
            .count(),
        1,
        "failed publish must not leave an extraction temp"
    );
}
