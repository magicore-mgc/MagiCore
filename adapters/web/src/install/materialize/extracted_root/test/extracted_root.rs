#![allow(clippy::unwrap_used)]
// extracted_root owner tests — kiểm thử owner extracted_root.
// These checks pin memo reuse without invoking extraction or touching package data.
// Kiểm thử xác nhận memo được dùng lại mà không bung hoặc sửa dữ liệu package.
use super::*;
use crate::install::materialize::test::support::{
    package_id, resolved_package, storage_handles, tempdir_real,
};

#[test]
fn extracted_root_lookup_reuses_memo_entry_before_extraction() {
    let temp = tempdir_real();
    let (layout, store, cache) = storage_handles(temp.path());
    let package = resolved_package(package_id("memoized@1.0.0"), true, vec![]);
    let root = temp.path().join("already-extracted");
    let mut roots = std::collections::HashMap::from([(package.id.clone(), root.clone())]);

    let result = extracted_root_for(&mut roots, &layout, &store, None, &cache, &package, 1);

    assert_eq!(result.unwrap(), root);
    assert_eq!(roots.len(), 1);
}
