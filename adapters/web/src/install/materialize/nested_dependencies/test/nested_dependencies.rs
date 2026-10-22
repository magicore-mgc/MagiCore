#![allow(clippy::unwrap_used)]
// nested dependency tests — kiểm thử dependency lồng.
// The depth guard must reject recursion before creating package directories.
// Giới hạn độ sâu phải chặn trước khi tạo thư mục package.
use super::*;
use crate::install::materialize::test::support::{
    package_id, resolved_package, storage_handles, tempdir_real,
};

#[test]
fn dependency_depth_over_limit_fails_before_materializing_files() {
    let temp = tempdir_real();
    let package_dir = temp.path().join("package");
    let (layout, store, cache) = storage_handles(temp.path());
    let package = resolved_package(package_id("deep@1.0.0"), true, vec![]);
    let mut scripts = Vec::new();

    let result = materialize_nested_dependencies(
        &package_dir,
        &package,
        &std::collections::HashMap::new(),
        &std::collections::HashMap::new(),
        &layout,
        &store,
        None,
        &cache,
        &mut std::collections::HashMap::new(),
        &mut std::collections::HashSet::new(),
        51,
        &mut scripts,
        1,
    );

    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("dependency graph too deep")
    );
    assert!(!package_dir.exists());
    assert!(scripts.is_empty());
}
