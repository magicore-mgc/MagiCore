//! Tests that a verified file handle stays bound to the opened CAS object.
//! Kiểm thử handle đã xác minh vẫn gắn với object CAS mở ban đầu.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::cas::store::ContentStore;

#[test]
fn returns_the_verified_file_from_offset_zero() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"exported from this handle").unwrap();

    let source = hash.cas_path(&store.root);
    let mut file = store.open_verified_blob(&hash, &source).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"exported from this handle");
}

#[test]
fn cache_miss_hashes_from_the_start_of_a_nonzero_handle() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"cache miss handle offset").unwrap();
    let source_path = hash.cas_path(&store.root);
    let mut source = File::open(&source_path).unwrap();
    source.seek(SeekFrom::End(0)).unwrap();

    let mut file = store
        .verify_opened_blob(&hash, &source_path, source)
        .unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"cache miss handle offset");
}

#[test]
fn memo_hit_rewinds_a_nonzero_source_handle() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"memo handle offset").unwrap();
    let source_path = hash.cas_path(&store.root);
    store.open_verified_blob(&hash, &source_path).unwrap();

    let mut source = File::open(&source_path).unwrap();
    source.seek(SeekFrom::End(0)).unwrap();
    let mut file = store
        .verify_opened_blob(&hash, &source_path, source)
        .unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"memo handle offset");
}

#[cfg(unix)]
#[test]
fn verified_handle_survives_source_path_replacement() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"trusted blob").unwrap();
    let source = hash.cas_path(&store.root);
    let moved = temp.path().join("moved.blob");
    let replacement = temp.path().join("replacement.blob");
    std::fs::write(&replacement, b"different bytes").unwrap();

    let mut file = store.open_verified_blob(&hash, &source).unwrap();
    std::fs::rename(&source, &moved).unwrap();
    symlink(&replacement, &source).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    assert_eq!(bytes.as_slice(), b"trusted blob");
}
