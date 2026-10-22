//! Byte import mode test.
//! Kiểm thử mode import byte.

use crate::cas::store::ContentStore;

#[test]
fn executable_mode_uses_a_distinct_content_address() {
    let temp = tempfile::tempdir().unwrap();
    let cas_root = temp.path().join("cas");
    let store = ContentStore::new(cas_root.clone()).unwrap();

    let regular = store.import_bytes_with_exec(b"same bytes", false).unwrap();
    let executable = store.import_bytes_with_exec(b"same bytes", true).unwrap();

    assert_eq!(regular.as_hex(), executable.as_hex());
    assert_ne!(regular, executable);
    assert_ne!(regular.cas_path(&cas_root), executable.cas_path(&cas_root));
    assert!(regular.cas_path(&cas_root).is_file());
    assert!(executable.cas_path(&cas_root).is_file());
}
