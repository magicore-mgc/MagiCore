//! Dedupe settings serde contract tests.
//! Test contract serde cho cấu hình dedupe.

use super::super::DedupeConfig;

#[test]
fn missing_prefer_defaults_to_false() {
    let config: DedupeConfig =
        serde_json::from_str("{}").expect("empty dedupe config should deserialize");
    assert!(!config.prefer);
    let serialized = serde_json::to_value(&config).expect("dedupe config should serialize");
    assert_eq!(serialized, serde_json::json!({"prefer": false}));
}

#[test]
fn explicit_prefer_value_roundtrips() {
    let config = DedupeConfig { prefer: true };
    let serialized = serde_json::to_value(&config).expect("dedupe config should serialize");
    assert_eq!(serialized, serde_json::json!({"prefer": true}));
    let restored: DedupeConfig =
        serde_json::from_value(serialized).expect("dedupe config should deserialize");
    assert!(restored.prefer);
}
