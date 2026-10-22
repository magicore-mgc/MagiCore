//! Registry source serde contract tests.
//! Test contract serde cho registry source.

use super::super::SourceConfig;

#[test]
fn required_identity_fields_keep_current_defaults_for_the_rest() {
    let config: SourceConfig =
        serde_json::from_str(r#"{"id":"internal","url":"https://registry.example"}"#)
            .expect("source identity fields should be sufficient");
    assert_eq!(config.id, "internal");
    assert_eq!(config.url, "https://registry.example");
    assert_eq!(config.ecosystem, "");
    assert_eq!(config.priority, 0);
    assert!(config.claims.is_empty());
    assert!(!config.trusted);
    assert!(config.allow_hosts.is_empty());
    assert!(config.allow_cidrs.is_empty());
    assert!(config.allow_protocols.is_empty());
}

#[test]
fn explicit_claim_and_trust_fields_roundtrip() {
    let config = SourceConfig {
        id: "private".to_string(),
        url: "https://packages.example".to_string(),
        ecosystem: "python".to_string(),
        priority: 10,
        claims: vec!["corp-*".to_string()],
        trusted: true,
        allow_hosts: vec!["packages.example".to_string()],
        allow_cidrs: vec!["10.0.0.0/8".to_string()],
        allow_protocols: vec!["https".to_string()],
    };
    let value = serde_json::to_value(&config).expect("source config should serialize");
    assert_eq!(
        serde_json::from_value::<SourceConfig>(value).expect("source config should deserialize"),
        config
    );
}
