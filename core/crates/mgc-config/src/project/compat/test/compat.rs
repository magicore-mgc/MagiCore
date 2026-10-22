//! Compatibility settings serde contract tests.
//! Test contract serde cho cấu hình tương thích.

use super::super::CompatConfig;

#[test]
fn missing_fields_keep_deny_by_default() {
    let config: CompatConfig =
        serde_json::from_str("{}").expect("empty compat config should deserialize");
    assert_eq!(config, CompatConfig::default());
    assert!(!config.allow_git_deps);
    assert_eq!(
        serde_json::to_value(config).expect("compat config should serialize"),
        serde_json::json!({"allow_git_deps": false, "git_hosts": []})
    );
}

#[test]
fn explicit_host_allowlist_roundtrips() {
    let config = CompatConfig {
        allow_git_deps: true,
        git_hosts: vec!["git.example.org".to_string()],
    };
    let value = serde_json::to_value(&config).expect("compat config should serialize");
    assert_eq!(
        value,
        serde_json::json!({"allow_git_deps": true, "git_hosts": ["git.example.org"]})
    );
    assert_eq!(
        serde_json::from_value::<CompatConfig>(value).expect("compat config should deserialize"),
        config
    );
}
