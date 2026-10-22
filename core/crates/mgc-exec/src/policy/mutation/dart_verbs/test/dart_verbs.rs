use super::super::DART_MUTATION_VERBS;

#[test]
fn catalog_keeps_the_exact_baseline_entries_and_order() {
    assert_eq!(
        DART_MUTATION_VERBS,
        &["get", "add", "remove", "upgrade", "downgrade"]
    );
}
