#![allow(clippy::unwrap_used)]
// strict materialization tests — kiểm thử strict materialization.
// This checks empty-graph setup without introducing package filesystem effects.
// Kiểm thử setup graph rỗng mà không tạo side effect của package.
use super::*;
use crate::install::materialize::test::support::{storage_handles, tempdir_real};

#[test]
fn strict_layout_creates_virtual_store_for_an_empty_graph() {
    let temp = tempdir_real();
    let modules = temp.path().join("node_modules");
    let (layout, store, cache) = storage_handles(temp.path());
    let graph = ResolvedGraph { packages: vec![] };
    let mut packages_with_scripts = Vec::new();

    materialize_strict_layout(
        &modules,
        &graph,
        &graph,
        &std::collections::HashMap::new(),
        &[],
        &layout,
        &store,
        None,
        &cache,
        &mut packages_with_scripts,
        &std::collections::HashMap::new(),
        1,
    )
    .unwrap();

    assert!(modules.join(".magicore").is_dir());
    assert!(packages_with_scripts.is_empty());
}
