//! Declared-hash cache-hit security regression test.
//! Test hồi quy bảo mật cache-hit với hash khai báo.

use crate::cas::store::ContentStore;

#[test]
fn rejects_wrong_content_even_when_declared_blob_is_already_cached() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let cached = store.import_bytes(b"payload B").unwrap();

    let result = store.import_bytes_with_hash(b"payload A", cached.as_hex(), false);

    assert!(result.is_err());
}

#[test]
fn valid_byte_import_repairs_a_corrupt_existing_blob() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    let store = ContentStore::new(root.clone()).unwrap();
    let payload = b"recover the addressed blob";
    let hash = store.import_bytes(payload).unwrap();
    std::fs::write(hash.cas_path(&root), b"corrupt entry").unwrap();

    let imported = store.import_bytes(payload).unwrap();

    assert_eq!(imported.as_hex(), hash.as_hex());
    assert_eq!(std::fs::read(hash.cas_path(&root)).unwrap(), payload);
    assert!(
        std::fs::read_dir(root.join("quarantine"))
            .unwrap()
            .next()
            .is_some()
    );
}

#[test]
fn independent_store_handles_import_duplicate_bytes_without_replacing_active_readers() {
    const WORKERS: usize = 16;
    const ROUNDS: usize = 16;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    let stores = (0..WORKERS)
        .map(|_| ContentStore::new(root.clone()).unwrap())
        .collect::<Vec<_>>();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(WORKERS));
    let handles = stores
        .into_iter()
        .map(|store| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut errors = Vec::new();
                for round in 0..ROUNDS {
                    let bytes = vec![round as u8; 256 * 1024];
                    barrier.wait();
                    for _ in 0..8 {
                        if let Err(error) = store.import_bytes(&bytes) {
                            errors.push(error.to_string());
                        }
                    }
                }
                errors
            })
        })
        .collect::<Vec<_>>();
    let errors = handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    assert!(
        errors.is_empty(),
        "Concurrent dedup import must not invalidate active verification: {errors:?}"
    );
    let store = ContentStore::new(root.clone()).unwrap();
    for round in 0..ROUNDS {
        let bytes = vec![round as u8; 256 * 1024];
        let hash = store.import_bytes(&bytes).unwrap();
        assert_eq!(std::fs::read(hash.cas_path(&root)).unwrap(), bytes);
    }
}

#[test]
fn streamed_and_byte_imports_share_duplicate_publication_coordination() {
    const WORKERS: usize = 8;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    let bytes = std::sync::Arc::new(vec![0x5a; 2 * crate::cas::write::STREAM_THRESHOLD]);
    let source = temp.path().join("source.bin");
    std::fs::write(&source, bytes.as_slice()).unwrap();
    let stores = (0..WORKERS)
        .map(|_| ContentStore::new(root.clone()).unwrap())
        .collect::<Vec<_>>();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(WORKERS));
    let handles = stores
        .into_iter()
        .enumerate()
        .map(|(worker, store)| {
            let barrier = barrier.clone();
            let bytes = bytes.clone();
            let source = source.clone();
            std::thread::spawn(move || {
                barrier.wait();
                let mut result = if worker % 2 == 0 {
                    store.import_bytes(&bytes)
                } else {
                    store.import_file_with_exec(&source, false)
                };
                for _ in 0..8 {
                    if result.is_ok() {
                        result = store.import_bytes(&bytes);
                    }
                }
                result
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(
            result.is_ok(),
            "Mixed import must preserve active verification: {result:?}"
        );
    }
    let hash = crate::cas::integrity::IntegrityHash::from_bytes(&bytes, false);
    assert_eq!(
        std::fs::read(hash.cas_path(&root)).unwrap(),
        bytes.as_slice()
    );
}
