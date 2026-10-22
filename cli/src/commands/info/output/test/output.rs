// JSON projection behavior tests.
// Test hành vi JSON projection.

use std::collections::HashMap;

use mgc_web_adapter::native::npm_registry::{PackageMetadata, VersionInfo};
use serde_json::Value;

use super::render_json;

fn version_info(version: &str) -> VersionInfo {
    VersionInfo {
        version: version.to_string(),
        dependencies: None,
        dev_dependencies: None,
        peer_dependencies: None,
        peer_dependencies_meta: HashMap::new(),
        optional_dependencies: None,
        os: None,
        cpu: None,
        dist: None,
    }
}

fn package_metadata() -> PackageMetadata {
    PackageMetadata {
        name: "sample".to_string(),
        description: Some("sample package".to_string()),
        versions: HashMap::from([
            ("2.0.0".to_string(), version_info("2.0.0")),
            ("1.0.0".to_string(), version_info("1.0.0")),
        ]),
        dist_tags: HashMap::from([("latest".to_string(), "2.0.0".to_string())]),
        time: HashMap::new(),
    }
}

#[test]
fn json_projection_keeps_schema_and_sorts_versions() {
    let rendered = render_json(
        &package_metadata(),
        "[Core: General / Library]",
        Some("^1.0.0".to_string()),
    )
    .unwrap();
    let value: Value = serde_json::from_str(&rendered).unwrap();

    assert_eq!(value.as_object().unwrap().len(), 7);
    assert_eq!(value["name"], "sample");
    assert_eq!(value["description"], "sample package");
    assert_eq!(value["core_support"], "[Core: General / Library]");
    assert_eq!(value["version_count"], 2);
    assert_eq!(value["versions"], serde_json::json!(["1.0.0", "2.0.0"]));
    assert_eq!(value["local_version"], "^1.0.0");
    assert_eq!(value["dist_tags"][0], serde_json::json!(["latest", "2.0.0"]));
}
