//! Tests for hook configuration defaults and TOML decoding.
//! Test mặc định cấu hình hook và giải mã TOML.

use super::super::HooksConfig;

#[test]
fn default_config_has_no_hooks() {
    assert!(HooksConfig::default().hooks.is_empty());
}

#[test]
fn config_deserializes_event_command_lists() {
    let config: HooksConfig =
        toml::from_str(r#"hooks = { "pre-install" = ["cargo check"] }"#).unwrap();
    assert_eq!(config.hooks["pre-install"], vec!["cargo check".to_string()]);
}
