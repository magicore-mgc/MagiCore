//! CocoaPods dependency extraction contract tests.
//! Test contract trích dependency CocoaPods.

use super::*;

#[test]
fn spec_dependencies_map_to_edges() {
    let spec: Value = serde_json::from_str(
        r#"{"name":"Firebase","version":"10.18.0","dependencies":{"FirebaseCore":["= 10.18.0"],"nanopb":["~> 2.30908.0"]}}"#,
    )
    .expect("test operation must succeed");
    let dependencies = spec_dependencies(&spec);
    assert!(dependencies.contains(&("FirebaseCore".to_string(), "= 10.18.0".to_string())));
    assert!(dependencies.contains(&("nanopb".to_string(), "~> 2.30908.0".to_string())));
}

#[test]
fn missing_or_non_object_dependencies_produce_no_edges() {
    assert!(spec_dependencies(&serde_json::json!({})).is_empty());
    assert!(spec_dependencies(&serde_json::json!({"dependencies": []})).is_empty());
}
