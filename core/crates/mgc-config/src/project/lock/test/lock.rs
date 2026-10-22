//! Lock settings serde contract tests.
//! Test contract serde cho cấu hình lock.

use super::super::LockConfig;

#[test]
fn absent_values_are_omitted() {
    let config: LockConfig =
        serde_json::from_str("{}").expect("empty lock config should deserialize");
    assert_eq!(config, LockConfig::default());
    assert_eq!(
        serde_json::to_value(config).expect("lock config should serialize"),
        serde_json::json!({})
    );
}

#[test]
fn explicit_settings_roundtrip() {
    let config = LockConfig {
        policy: Some("require".to_string()),
        acquire_timeout_ms: Some(2_000),
        tmp_grace_secs: Some(30),
        schema: Some("4".to_string()),
    };
    let value = serde_json::to_value(&config).expect("lock config should serialize");
    assert_eq!(
        value,
        serde_json::json!({
            "policy": "require",
            "acquire_timeout_ms": 2000,
            "tmp_grace_secs": 30,
            "schema": "4"
        })
    );
    assert_eq!(
        serde_json::from_value::<LockConfig>(value).expect("lock config should deserialize"),
        config
    );
}
