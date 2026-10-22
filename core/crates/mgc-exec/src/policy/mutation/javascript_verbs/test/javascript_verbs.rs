use super::super::JAVASCRIPT_MUTATION_VERBS;

#[test]
fn catalog_keeps_the_exact_baseline_entries_and_order() {
    assert_eq!(
        JAVASCRIPT_MUTATION_VERBS,
        &[
            "install",
            "ci",
            "add",
            "remove",
            "uninstall",
            "update",
            "upgrade",
            "publish",
            "unpublish",
            "link",
            "unlink",
            "dedupe",
            "prune",
            "rebuild"
        ]
    );
}
