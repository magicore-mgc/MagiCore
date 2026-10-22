//! Contract tests for default execution profiles.
//! Test contract cho execution profile mặc định.

use super::super::execution::{ProjectExecutionConfig, default_execution_for};

#[test]
fn default_profile_matches_existing_shell_defaults() {
    let profile = ProjectExecutionConfig::default();

    assert_eq!(profile.architecture, "rust-first");
    assert_eq!(profile.lane, "compatibility-shell");
    assert_eq!(profile.compatibility_layer, "js");
    assert!(profile.native_targets.is_empty());
}

#[test]
fn serde_missing_fields_keep_the_existing_execution_defaults() {
    let profile: ProjectExecutionConfig =
        serde_json::from_str("{}").expect("empty profile uses field defaults");

    assert_eq!(profile, ProjectExecutionConfig::default());
}

#[test]
fn web_default_recognizes_trimmed_case_insensitive_typescript_features() {
    let profile = default_execution_for("web", &[" TypeScript ".to_string()]);
    let shorthand = default_execution_for("web", &["ts".to_string()]);

    assert_eq!(profile.architecture, "rust-first");
    assert_eq!(profile.lane, "compatibility-shell");
    assert_eq!(profile.compatibility_layer, "ts");
    assert_eq!(shorthand.compatibility_layer, "ts");
    assert_eq!(
        profile.native_targets,
        vec![
            "frontend-executable".to_string(),
            "backend-executable".to_string(),
            "wasm-bridge".to_string(),
        ]
    );
}

#[test]
fn native_first_cores_keep_the_binary_lane() {
    for ecosystem in ["game", "app", "lib"] {
        let profile = default_execution_for(ecosystem, &[]);

        assert_eq!(profile.architecture, "native-first");
        assert_eq!(profile.lane, "native-ready");
        assert_eq!(profile.compatibility_layer, "none");
        assert_eq!(profile.native_targets, vec!["binary".to_string()]);
    }
}

#[test]
fn other_ecosystems_keep_the_existing_empty_target_fallback() {
    for ecosystem in ["ai", "cloud", "clo", "iot", "cicd", "unknown"] {
        let profile = default_execution_for(ecosystem, &[]);

        assert_eq!(profile.architecture, "rust-first");
        assert_eq!(profile.lane, "compatibility-shell");
        assert_eq!(profile.compatibility_layer, "none");
        assert!(profile.native_targets.is_empty());
    }
}
