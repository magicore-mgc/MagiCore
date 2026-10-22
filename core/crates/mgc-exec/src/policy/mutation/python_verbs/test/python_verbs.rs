use super::super::PYTHON_MUTATION_VERBS;

#[test]
fn catalog_keeps_the_exact_baseline_entries_and_order() {
    assert_eq!(
        PYTHON_MUTATION_VERBS,
        &[
            "install",
            "add",
            "remove",
            "uninstall",
            "update",
            "upgrade",
            "sync",
            "lock"
        ]
    );
}
