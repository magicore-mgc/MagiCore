use super::super::COMPOSER_MUTATION_VERBS;

#[test]
fn catalog_keeps_the_exact_baseline_entries_and_order() {
    assert_eq!(
        COMPOSER_MUTATION_VERBS,
        &["install", "update", "require", "remove"]
    );
}
