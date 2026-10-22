//! Export Lifecycle tests. / Test export lifecycle.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::{ContentStore, IntegrityHash};
use std::fs;

pub(crate) fn exported_file_mutation_does_not_modify_store_blob() {
    let root = tmp_store_dir("export-isolation");
    let store = ContentStore::new(root.clone()).unwrap();

    let data = b"immutable-content".to_vec();
    let hash = store.import_bytes(&data).unwrap();

    let export_dir = root.join("export");
    fs::create_dir_all(&export_dir).unwrap();
    let dest = export_dir.join("linked.txt");

    store.export_to(&hash, &dest).unwrap();

    // Hardlink would share the inode — mutating the export must NOT change
    // the store blob. (The old hardlink export failed exactly here.)
    // Hardlink sẽ dùng chung inode — sửa file export KHÔNG được làm thay đổi
    // blob trong store. (Export hardlink cũ fail ngay tại đây.)
    let before = fs::read(hash.cas_path(&root)).unwrap();
    fs::write(&dest, b"MUTATED-BY-PROJECT").unwrap();
    let after = fs::read(hash.cas_path(&root)).unwrap();

    assert_eq!(
        before, after,
        "BUG: project-side mutation poisoned the shared store blob (writable hardlink still present?)"
    );
    assert_eq!(before, b"immutable-content".to_vec());

    // nlink check (unix): exported file must have link count 1.
    // Kiểm tra nlink (unix): file export phải có link count 1.
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta = fs::metadata(&dest).unwrap();
        assert_eq!(
            meta.nlink(),
            1,
            "exported file must be an independent copy, not a hardlink"
        );
    }
}

pub(crate) fn export_twice_is_idempotent() {
    let root = tmp_store_dir("export-idempotent");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"same-bytes").unwrap();

    let dest = root.join("out").join("file.txt");
    store.export_to(&hash, &dest).unwrap();
    store.export_to(&hash, &dest).unwrap();
    assert_eq!(fs::read(&dest).unwrap(), b"same-bytes".to_vec());
}

pub(crate) fn export_missing_blob_fails_not_found() {
    let root = tmp_store_dir("export-missing");
    let store = ContentStore::new(root.clone()).unwrap();
    let ghost = IntegrityHash::from_bytes(b"never-imported", false);
    let res = store.export_to(&ghost, &root.join("nowhere.txt"));
    assert!(res.is_err());
}

#[cfg(unix)]
pub(crate) fn export_to_existing_destination_repairs_executable_mode() {
    // Export an EXECUTABLE blob onto an existing 0644 destination with the
    // SAME bytes: export_to must succeed AND repair the mode to 0755 —
    // identical bytes alone are not a complete export (audit P1-3).
    // (Export blob EXECUTABLE lên đích 0644 đã tồn tại với CÙNG bytes:
    // export_to phải thành công VÀ sửa mode thành 0755 — bytes giống nhau
    // một mình chưa phải export hoàn chỉnh (P1-3 audit).)
    use std::os::unix::fs::PermissionsExt;
    let root = tmp_store_dir("exec-mode-repair");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store
        .import_bytes_with_exec(b"#!/bin/sh\necho hi\n", true)
        .unwrap();

    let dest = root.join("out").join("script.sh");
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    fs::write(&dest, b"#!/bin/sh\necho hi\n").unwrap(); // same bytes, 0644
    fs::set_permissions(&dest, fs::Permissions::from_mode(0o644)).unwrap();

    store.export_to(&hash, &dest).unwrap();

    let mode = fs::metadata(&dest).unwrap().permissions().mode() & 0o777;
    assert_eq!(
        mode, 0o755,
        "existing destination mode must be repaired to 0755"
    );
}

pub(crate) fn export_destination_under_symlinked_dir_is_rejected() {
    let root = tmp_store_dir("symlink-ancestor");
    let store = ContentStore::new(root.clone()).unwrap();

    let hash = store.import_bytes(b"symlinked-target").unwrap();

    let real_dir = root.join("real");
    let link_dir = root.join("linkdir");
    fs::create_dir_all(&real_dir).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&real_dir, &link_dir).unwrap();

    let dest = link_dir.join("escape.txt");
    let res = store.export_to(&hash, &dest);
    assert!(
        res.is_err(),
        "export through a symlinked directory must be refused"
    );
}
