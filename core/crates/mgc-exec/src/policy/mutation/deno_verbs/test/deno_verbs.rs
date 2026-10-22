use super::super::DENO_MUTATION_VERBS;

#[test]
fn catalog_keeps_the_exact_baseline_entries_and_order() {
    assert_eq!(
        DENO_MUTATION_VERBS,
        &["add", "remove", "install", "publish", "upgrade", "update"]
    );
}
