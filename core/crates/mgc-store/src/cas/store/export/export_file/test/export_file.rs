//! Export File source regression test.
//! Test hồi quy source cho owner export file.

use std::fs;

use crate::cas::store::ContentStore;

#[test]
fn export_is_independent_and_retains_executable_mode() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let cas_root = root.join("cas");
    let store = ContentStore::new(cas_root.clone()).unwrap();
    let hash = store.import_bytes_with_exec(b"source", true).unwrap();
    let dest = root.join("project").join("tool");

    store.export_to(&hash, &dest).unwrap();

    assert_eq!(fs::read(&dest).unwrap(), b"source");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_ne!(fs::metadata(&dest).unwrap().permissions().mode() & 0o111, 0);
    }
    fs::write(&dest, b"project edit").unwrap();
    assert_eq!(fs::read(hash.cas_path(&cas_root)).unwrap(), b"source");
}
