//! Tests for ProjectConfig serde defaults and optional-field omission.
//! Test default serde và bỏ field tùy chọn của ProjectConfig.

use crate::project::ProjectConfig;

#[test]
fn minimal_project_toml_uses_schema_defaults() {
    let config: ProjectConfig = toml::from_str("name = \"demo\"\necosystem = \"web\"\n").unwrap();
    assert_eq!(config.version, "0.1.0");
    assert!(config.mode.is_empty());
    assert!(config.frameworks.is_empty());
    assert!(config.lib.is_none());
}

#[test]
fn unset_optional_core_sections_are_omitted_when_serialized() {
    let config = ProjectConfig::new("demo", "web");
    let serialized = toml::to_string(&config).unwrap();
    let document: toml::Value = toml::from_str(&serialized).unwrap();
    for field in [
        "lib", "game", "iot", "cloud", "cicd", "app", "ai", "security", "lock", "trust", "compat",
        "sources", "scripts", "script",
    ] {
        assert!(
            document.get(field).is_none(),
            "unset {field} must be omitted"
        );
    }
}
