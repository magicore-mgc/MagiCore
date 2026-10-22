//! Direct file import test.
//! Test import file trực tiếp.

use std::fs;

use crate::cas::store::ContentStore;
use crate::cas::write::STREAM_THRESHOLD;

#[test]
fn imports_regular_file_to_its_content_addressed_path() {
    let temp = tempfile::tempdir().unwrap();
    let cas_root = temp.path().join("cas");
    let store = ContentStore::new(cas_root.clone()).unwrap();
    let source = temp.path().join("payload.bin");
    fs::write(&source, b"file payload").unwrap();

    let hash = store.import_file(&source).unwrap();

    assert_eq!(fs::read(hash.cas_path(&cas_root)).unwrap(), b"file payload");
}

#[test]
fn streamed_file_import_repairs_a_corrupt_existing_blob() {
    let temp = tempfile::tempdir().unwrap();
    let cas_root = temp.path().join("cas");
    let store = ContentStore::new(cas_root.clone()).unwrap();
    let source = temp.path().join("large-payload.bin");
    let payload = vec![b'x'; STREAM_THRESHOLD + 1];
    fs::write(&source, &payload).unwrap();
    let hash = store.import_file(&source).unwrap();
    fs::write(hash.cas_path(&cas_root), b"corrupt streamed entry").unwrap();

    let repaired = store.import_file(&source).unwrap();

    assert_eq!(repaired.as_hex(), hash.as_hex());
    assert_eq!(fs::read(hash.cas_path(&cas_root)).unwrap(), payload);
    assert!(
        fs::read_dir(cas_root.join("quarantine"))
            .unwrap()
            .next()
            .is_some()
    );
}
