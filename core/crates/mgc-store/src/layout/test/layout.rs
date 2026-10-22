//! Store layout path-contract tests.
//! Kiểm tra hợp đồng ánh xạ đường dẫn của store layout.

use super::super::Layout;
use std::path::PathBuf;

#[test]
fn root_returns_the_supplied_path() {
    let root = PathBuf::from("workspace/store");
    let layout = Layout::new(root.clone());

    assert_eq!(layout.root(), root.as_path());
}

#[test]
fn child_paths_keep_the_store_layout_contract() {
    let layout = Layout::new(PathBuf::from("workspace/store"));

    assert_eq!(layout.cas_dir(), PathBuf::from("workspace/store/cas"));
    assert_eq!(layout.cache_dir(), PathBuf::from("workspace/store/cache"));
    assert_eq!(layout.db_path(), PathBuf::from("workspace/store/store.db"));
    assert_eq!(layout.temp_dir(), PathBuf::from("workspace/store/tmp"));
    assert_eq!(layout.logs_dir(), PathBuf::from("workspace/store/logs"));
    assert_eq!(layout.locks_dir(), PathBuf::from("workspace/store/locks"));
    assert_eq!(
        layout.virtual_store_dir(),
        PathBuf::from("workspace/store/virtual_store")
    );
    assert_eq!(
        layout.index_msgpack_path(),
        PathBuf::from("workspace/store/index.msgpack")
    );
}
