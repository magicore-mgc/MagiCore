//! Tests for stable ProjectConfig constructor defaults.
//! Test default ổn định của constructor ProjectConfig.

use crate::project::ProjectConfig;

#[test]
fn constructor_sets_shared_and_core_sections_to_existing_defaults() {
    let config = ProjectConfig::new("demo", "app");
    assert_eq!(config.name, "demo");
    assert_eq!(config.version, "0.1.0");
    assert_eq!(config.ecosystem, "app");
    assert!(config.frameworks.is_empty());
    assert_eq!(config.execution.architecture, "native-first");
    assert_eq!(config.execution.lane, "native-ready");
    assert_eq!(config.execution.compatibility_layer, "none");
    assert_eq!(config.execution.native_targets, vec!["binary".to_string()]);
    assert!(config.registries.is_empty());
    assert!(config.patches.is_empty());
    assert!(config.lib.is_none());
    assert!(config.app.is_none());
}
