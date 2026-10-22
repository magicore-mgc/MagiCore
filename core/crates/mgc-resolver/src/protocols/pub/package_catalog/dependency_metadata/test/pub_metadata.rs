//! Package metadata policy regression tests.
//! Test hồi quy policy metadata package.

use super::super::parse_pub_dependencies;
use mgc_types::MgError;
use std::collections::HashMap;

#[test]
fn pub_metadata_rejects_non_registry_dependencies_instead_of_dropping_them() {
    for source in ["path", "git", "hosted"] {
        let value = match source {
            "path" => serde_json::json!({"path": "../local"}),
            "git" => serde_json::json!({"git": {"url": "https://example.invalid/repo.git"}}),
            _ => serde_json::json!({"hosted": "https://packages.invalid", "version": "^1.0.0"}),
        };
        let dependencies = HashMap::from([("local_pkg".to_string(), value)]);
        let error = parse_pub_dependencies(&dependencies, &mut vec![])
            .expect_err("test operation must fail");
        assert!(matches!(error, MgError::Unsupported { .. }));
        assert!(error.to_string().contains(source));
    }
}

#[test]
fn pub_metadata_excludes_sdk_packages_but_keeps_registry_edges() {
    let dependencies = HashMap::from([
        ("flutter_test".to_string(), serde_json::json!("any")),
        ("http".to_string(), serde_json::json!("^1.0.0")),
    ]);
    let mut markers = vec![];
    let parsed =
        parse_pub_dependencies(&dependencies, &mut markers).expect("test operation must succeed");
    assert_eq!(parsed, vec![("http".to_string(), "^1.0.0".to_string())]);
    assert_eq!(markers, vec!["sdk-owned:flutter_test"]);
}
