#![allow(clippy::unwrap_used)]
// graph owner tests — kiểm thử owner graph.
// These checks pin direct-version selection and ordered graph filtering.
// Kiểm thử khóa chọn phiên bản trực tiếp và lọc graph giữ nguyên thứ tự.
use super::*;
use crate::install::materialize::test::support::{package_id, resolved_package};

#[test]
fn root_selection_prefers_direct_version_and_keeps_unique_transitive_packages() {
    let older = package_id("postcss@8.5.23");
    let newer = package_id("postcss@8.5.28");
    let plugin = package_id("plugin@1.0.0");
    let graph = ResolvedGraph {
        packages: vec![
            resolved_package(older.clone(), true, vec![]),
            resolved_package(newer.clone(), false, vec![]),
            resolved_package(plugin.clone(), false, vec![newer.clone()]),
        ],
    };

    let roots = select_root_packages(&graph);
    let root_ids: Vec<_> = roots.iter().map(|package| package.id.clone()).collect();

    assert!(root_ids.contains(&older));
    assert!(root_ids.contains(&plugin));
    assert!(!root_ids.contains(&newer));
}

#[test]
fn graph_filter_preserves_order_and_removes_only_excluded_ids() {
    let first = resolved_package(package_id("first@1.0.0"), true, vec![]);
    let second = resolved_package(package_id("second@2.0.0"), false, vec![]);
    let graph = ResolvedGraph {
        packages: vec![first.clone(), second.clone()],
    };
    let filtered = graph_without_packages(&graph, &std::collections::HashSet::from([first.id]));

    assert_eq!(filtered.packages.len(), 1);
    assert_eq!(filtered.packages[0].id, second.id);
}
