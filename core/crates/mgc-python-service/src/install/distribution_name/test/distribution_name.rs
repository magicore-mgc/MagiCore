//! PEP 503 distribution-name tests.
//! Test tên distribution PEP 503.

use super::super::normalize_python_distribution_name;

#[test]
fn folds_case_and_collapses_any_separator_run_to_one_hyphen() {
    assert_eq!(
        normalize_python_distribution_name("My_Package.Name"),
        "my-package-name"
    );
    assert_eq!(normalize_python_distribution_name("A...B___C"), "a-b-c");
}

#[test]
fn ignores_separators_at_the_edges() {
    assert_eq!(normalize_python_distribution_name("--.Foo._"), "foo");
}
