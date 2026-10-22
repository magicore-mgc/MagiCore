//! Tests membership state before and after removal.
//! Kiểm thử trạng thái membership trước và sau khi xóa.

use crate::cas::store::ContentStore;

#[test]
fn removal_clears_the_blob_from_membership() {
    let temp = tempfile::tempdir().unwrap();
    let store = ContentStore::new(temp.path().join("cas")).unwrap();
    let hash = store.import_bytes(b"membership payload").unwrap();

    assert!(store.contains(&hash));
    store.remove(&hash).unwrap();
    assert!(!store.contains(&hash));
}
