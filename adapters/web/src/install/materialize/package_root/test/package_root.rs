#![allow(clippy::unwrap_used)]
// package-root tests — kiểm thử package-root.
// A cached extraction is reusable only while its package manifest is present.
// Chỉ dùng lại extracted-root đã cache khi manifest package còn tồn tại.
use super::*;
use crate::install::materialize::test::support::{
    package_id, resolved_package, storage_handles, tempdir_real,
};

#[test]
fn package_root_reuses_cached_root_with_manifest_before_tarball_lookup() {
    let temp = tempdir_real();
    let (layout, store, cache) = storage_handles(temp.path());
    let package = resolved_package(package_id("root-cache@1.0.0"), true, vec![]);
    let root = temp.path().join("extracted/root-cache");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("package.json"),
        br#"{"name":"root-cache","version":"1.0.0"}"#,
    )
    .unwrap();
    let roots = std::collections::HashMap::from([(package.id.clone(), root.clone())]);

    let result = materialization_package_root(&layout, &store, None, &cache, &roots, &package, 1);

    assert_eq!(result.unwrap(), root);
}
