//! Trust settings serde contract tests.
//! Test contract serde cho cấu hình trust.

use super::super::TrustConfig;

#[test]
fn missing_keys_default_to_an_empty_list() {
    let config: TrustConfig =
        serde_json::from_str("{}").expect("empty trust config should deserialize");
    assert_eq!(config, TrustConfig::default());
    assert_eq!(
        serde_json::to_value(config).expect("trust config should serialize"),
        serde_json::json!({"keys": []})
    );
}

#[test]
fn explicit_key_ids_roundtrip() {
    let config = TrustConfig {
        keys: vec!["0123456789abcdef".to_string()],
    };
    let value = serde_json::to_value(&config).expect("trust config should serialize");
    assert_eq!(value, serde_json::json!({"keys": ["0123456789abcdef"]}));
    assert_eq!(
        serde_json::from_value::<TrustConfig>(value).expect("trust config should deserialize"),
        config
    );
}
