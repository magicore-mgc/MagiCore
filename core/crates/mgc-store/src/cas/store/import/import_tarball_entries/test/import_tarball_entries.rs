//! Tarball-entry import test.
//! Kiểm thử import entry tarball.

use crate::cas::integrity::TarballEntry;
use crate::cas::store::ContentStore;

#[test]
fn imports_entry_bytes_to_the_returned_cas_address() {
    let temp = tempfile::tempdir().unwrap();
    let cas_root = temp.path().join("cas");
    let store = ContentStore::new(cas_root.clone()).unwrap();
    let data = b"archive member".to_vec();

    let hashes = store
        .import_tarball_entries(vec![TarballEntry {
            path: "pkg/file.bin".into(),
            data: data.clone(),
            executable: false,
        }])
        .unwrap();

    assert_eq!(hashes.len(), 1);
    assert_eq!(std::fs::read(hashes[0].cas_path(&cas_root)).unwrap(), data);
}
