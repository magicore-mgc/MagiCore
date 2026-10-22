//! Verify coordinator output keeps root first and package names sorted.
//! Kiểm tra coordinator đặt root đầu tiên và sắp package theo tên.

use super::*;

#[test]
fn package_graph_keeps_root_first_and_sorted_entries() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pubspec.yaml"),
        "name: sample\nversion: 2.0.0\ndependencies:\n  zeta: ^1.0.0\n  alpha: ^1.0.0\n",
    )
    .unwrap();

    let graph = super::build_flutter_package_graph_inner(
        &support::graph(&["zeta", "alpha"]),
        project.path(),
        None,
        &[],
    )
    .unwrap();

    assert_eq!(graph.roots, vec!["sample".to_string()]);
    assert_eq!(graph.packages[0].name, "sample");
    assert_eq!(
        graph.packages[0].dependencies,
        vec!["alpha".to_string(), "zeta".to_string()]
    );
    assert_eq!(graph.packages[1].name, "alpha");
    assert_eq!(graph.packages[2].name, "zeta");
}
