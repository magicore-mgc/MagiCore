//! Export Batch source regression test.
//! Test hồi quy source cho owner export batch.

use std::fs;

use crate::cas::store::ContentStore;

#[test]
fn durable_and_rebuildable_batches_publish_files() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let store = ContentStore::new(root.join("cas")).unwrap();
    let first = store.import_bytes(b"first").unwrap();
    let second = store.import_bytes(b"second").unwrap();
    let durable = root.join("durable").join("first.bin");
    let rebuildable = root.join("cache").join("second.bin");

    store
        .export_batch_to(vec![(first, durable.clone())])
        .unwrap();
    store
        .export_batch_to_rebuildable_root(vec![(second, rebuildable.clone())])
        .unwrap();

    assert_eq!(fs::read(durable).unwrap(), b"first");
    assert_eq!(fs::read(rebuildable).unwrap(), b"second");
}
