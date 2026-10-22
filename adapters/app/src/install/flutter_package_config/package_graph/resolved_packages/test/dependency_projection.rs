//! Verify resolved dependencies are stable, sorted and deduplicated.
//! Kiểm tra dependency resolved ổn định, sắp xếp và bỏ trùng.

use super::*;

#[test]
fn resolved_package_dependencies_are_sorted_and_deduplicated() {
    let graph = support::graph(vec![support::package(
        "consumer",
        "1.0.0",
        &[("zeta", "1.0.0"), ("alpha", "1.0.0"), ("zeta", "1.0.0")],
    )]);

    let entries = super::resolved_package_entries(&graph, "root").unwrap();

    assert_eq!(
        entries["consumer"].dependencies,
        vec!["alpha".to_string(), "zeta".to_string()]
    );
}
