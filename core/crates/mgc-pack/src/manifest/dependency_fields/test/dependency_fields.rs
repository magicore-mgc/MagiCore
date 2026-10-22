//! Tests for publish dependency projection.
//! Test projection dependency cho publish.

use super::dep_fields;

#[test]
fn keeps_dependency_groups_and_omits_unrelated_fields() {
    let manifest = serde_json::json!({
        "dependencies": {"runtime": "^1"},
        "devDependencies": {"build": "^2"},
        "peerDependencies": {"host": "^3"},
        "optionalDependencies": {"optional": "^4"},
        "scripts": {"test": "test"}
    });

    let fields = dep_fields(&manifest);

    assert_eq!(fields.len(), 4);
    assert_eq!(fields["dependencies"]["runtime"], "^1");
    assert!(fields.get("scripts").is_none());
}
