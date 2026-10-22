//! Import Batch tests. / Test import batch.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::{ContentStore, IntegrityHash};
use std::fs;

pub(crate) fn batched_cas_import_and_export_preserve_independent_verified_files() {
    let root = tmp_store_dir("batch-import-export");
    let store = ContentStore::new(root.clone()).unwrap();
    let files = [
        (b"first package file".as_slice(), false),
        (b"#!/bin/sh\nsecond package file\n".as_slice(), true),
    ];

    let hashes = store.import_bytes_with_exec_batch(files).unwrap();
    assert_eq!(hashes.len(), files.len());
    assert_eq!(hashes[0], IntegrityHash::from_bytes(files[0].0, false));
    assert_eq!(hashes[1], IntegrityHash::from_bytes(files[1].0, true));

    let first_export = root.join("project/pkg/first.txt");
    let second_export = root.join("project/pkg/bin/second.sh");
    store
        .export_batch_to(vec![
            (hashes[0].clone(), first_export.clone()),
            (hashes[1].clone(), second_export.clone()),
        ])
        .unwrap();

    assert_eq!(fs::read(&first_export).unwrap(), files[0].0);
    assert_eq!(fs::read(&second_export).unwrap(), files[1].0);
    fs::write(&first_export, b"project mutation").unwrap();
    let stored_hash = store.verify(&hashes[0].cas_path(&root)).unwrap();
    assert_eq!(stored_hash, hashes[0]);
    assert_eq!(fs::read(hashes[0].cas_path(&root)).unwrap(), files[0].0);
}

pub(crate) fn rebuildable_batch_import_preserves_hashes_modes_and_export_validation() {
    let root = tmp_store_dir("rebuildable-batch-import");
    let store = ContentStore::new(root.clone()).unwrap();
    let files = [
        (b"rebuildable package file".as_slice(), false),
        (b"#!/bin/sh\nrebuildable executable\n".as_slice(), true),
    ];

    let hashes = store
        .import_rebuildable_bytes_with_exec_batch(files)
        .unwrap();

    assert_eq!(hashes[0], IntegrityHash::from_bytes(files[0].0, false));
    assert_eq!(hashes[1], IntegrityHash::from_bytes(files[1].0, true));
    for (hash, (data, _)) in hashes.iter().zip(files) {
        assert_eq!(fs::read(hash.cas_path(&root)).unwrap(), data);
    }

    let exported = root.join("project/pkg/executable.sh");
    store.export_to(&hashes[1], &exported).unwrap();
    assert_eq!(fs::read(&exported).unwrap(), files[1].0);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_ne!(
            fs::metadata(exported).unwrap().permissions().mode() & 0o111,
            0
        );
    }

    fs::write(hashes[0].cas_path(&root), b"tampered rebuildable blob").unwrap();
    let rejected_export = root.join("project/pkg/rejected.txt");
    assert!(store.export_to(&hashes[0], &rejected_export).is_err());
    assert!(!rejected_export.exists());
    assert!(
        fs::read_dir(root.join("quarantine"))
            .unwrap()
            .next()
            .is_some()
    );
}

pub(crate) fn rebuildable_root_batch_export_preserves_integrity_and_independence() {
    let root = tmp_store_dir("staging-batch-export");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"verified staging output";
    let hash = store.import_bytes(data).unwrap();
    let exported = root.join("staging/package/file.txt");

    store
        .export_batch_to_rebuildable_root(vec![(hash.clone(), exported.clone())])
        .unwrap();

    assert_eq!(fs::read(&exported).unwrap(), data);
    fs::write(&exported, b"mutable staging copy").unwrap();
    assert_eq!(store.verify(&hash.cas_path(&root)).unwrap(), hash);
    assert_eq!(fs::read(hash.cas_path(&root)).unwrap(), data);
}
