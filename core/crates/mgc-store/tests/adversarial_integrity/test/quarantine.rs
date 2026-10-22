//! Quarantine tests. / Test quarantine.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::integrity::TarballEntry;
use mgc_store::cas::{ContentStore, IntegrityHash};
use std::fs;

pub(crate) fn tampered_existing_blob_fails_import_and_is_quarantined() {
    let root = tmp_store_dir("tampered-hit");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"hello-cas".to_vec();

    let hash = store.import_bytes(&data).unwrap();
    let blob_path = hash.cas_path(&root);

    // Tamper with the stored blob directly on disk.
    // Sửa blob trên đĩa trực tiếp — mô phỏng store bị đầu độc.
    fs::write(&blob_path, b"POISONED!!!").unwrap();

    // A verified replacement may repair the entry, but must preserve evidence.
    // Bản thay thế đã xác minh có thể sửa entry, nhưng phải giữ bằng chứng.
    let repaired = store
        .import_bytes(&data)
        .expect("verified source bytes should repair the poisoned entry");
    assert_eq!(repaired, hash);
    assert_eq!(fs::read(&blob_path).unwrap(), data);

    // The poisoned blob must have been moved to quarantine.
    // Blob bị đầu độc phải được chuyển vào quarantine.
    let qdir = root.join("quarantine");
    let entries: Vec<_> = fs::read_dir(&qdir)
        .map(|rd| rd.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    assert!(
        !entries.is_empty(),
        "corrupt blob must land in quarantine for forensics"
    );

    // After quarantine, re-import succeeds (self-healing repair).
    // Sau khi bị cách ly, import lại thành công (tự sửa).
    let repaired = store.import_bytes(&data).unwrap();
    assert_eq!(repaired.as_hex(), hash.as_hex());
}

pub(crate) fn batched_export_of_tampered_blob_is_rejected_and_quarantined() {
    let root = tmp_store_dir("batch-export-tampered-source");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"expected batch export bytes").unwrap();
    fs::write(hash.cas_path(&root), b"poisoned batch export bytes").unwrap();
    let exported = root.join("project/pkg/file.txt");

    let result = store.export_batch_to(vec![(hash, exported.clone())]);
    assert!(result.is_err(), "tampered CAS content must fail closed");
    assert!(!exported.exists(), "tampered bytes must not be published");
    let quarantine = fs::read_dir(root.join("quarantine")).unwrap();
    assert!(
        quarantine.count() > 0,
        "corrupt CAS content must be quarantined"
    );
}

#[cfg(unix)]
pub(crate) fn batched_export_rejects_cas_symlink_even_when_target_bytes_match() {
    use std::os::unix::fs::symlink;

    let root = tmp_store_dir("batch-export-cas-symlink");
    let store = ContentStore::new(root.clone()).unwrap();
    let data = b"matching bytes behind a CAS symlink";
    let hash = store.import_bytes(data).unwrap();
    let blob = hash.cas_path(&root);
    let external_target = root.join("external-target.bin");
    fs::write(&external_target, data).unwrap();
    fs::remove_file(&blob).unwrap();
    symlink(&external_target, &blob).unwrap();
    let exported = root.join("project/pkg/file.txt");

    let result = store.export_batch_to(vec![(hash, exported.clone())]);
    assert!(result.is_err(), "CAS symlinks must fail closed");
    assert!(
        !exported.exists(),
        "CAS symlink content must not be published"
    );
}

pub(crate) fn tampered_tarball_dedup_entry_fails() {
    let root = tmp_store_dir("tarball-dedup");
    let store = ContentStore::new(root.clone()).unwrap();

    let data = b"tar-entry".to_vec();
    let entries = vec![TarballEntry {
        path: "a.txt".into(),
        data: data.clone(),
        executable: false,
    }];
    store.import_tarball_entries(entries).unwrap();

    let hash = IntegrityHash::from_bytes(&data, false);
    fs::write(hash.cas_path(&root), b"corrupted").unwrap();

    let entries2 = vec![TarballEntry {
        path: "a.txt".into(),
        data: data.clone(),
        executable: false,
    }];
    let repaired = store
        .import_tarball_entries(entries2)
        .expect("verified tarball entry should repair the poisoned CAS object");
    assert_eq!(repaired, vec![hash.clone()]);
    assert_eq!(fs::read(hash.cas_path(&root)).unwrap(), data);
    assert!(fs::read_dir(root.join("quarantine")).unwrap().count() > 0);
}
