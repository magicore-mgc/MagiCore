use super::super::*;

#[test]
fn collect_leaves_flattens_pages_and_leaf_arrays() {
    let doc: Value = serde_json::from_str(
        r#"{"items":[{"items":[{"catalogEntry":{"version":"1.0.0"}},{"catalogEntry":{"version":"2.0.0"}}]}]}"#,
    )
    .expect("test operation must succeed");
    let mut leaves = Vec::new();
    collect_leaves(&doc, &mut leaves);
    assert_eq!(leaves.len(), 2);
}
