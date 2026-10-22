//! Directory Sync source regression test.
//! Test hồi quy source cho owner directory sync.

use std::collections::BTreeSet;

#[test]
fn syncs_the_supplied_parent_directory_set() {
    let temp = tempfile::tempdir().unwrap();
    let dirs = BTreeSet::from([temp.path().to_path_buf()]);

    super::super::directory_sync::sync_parent_directories(dirs).unwrap();
}
