//! Cargo `[env]` schema tests.
//! (Kiểm thử schema `[env]` của Cargo.)

use std::path::Path;

#[test]
fn env_settings_preserve_string_values() {
    let path = Path::new("project/Cargo.toml");
    let value: toml::Value = toml::from_str("SAFE_SETTING = \"enabled\"\n").unwrap();
    let settings = super::parse_env_settings(path, &value).unwrap();

    assert_eq!(settings.get("SAFE_SETTING"), Some(&"enabled".to_string()));
}

#[test]
fn env_settings_reject_reserved_path_key() {
    let path = Path::new("project/Cargo.toml");
    let value: toml::Value = toml::from_str("PATH = \"/tmp/injected\"\n").unwrap();
    let error = super::parse_env_settings(path, &value).unwrap_err();

    assert_eq!(
        error.to_string(),
        "optimizer Cargo configuration 'project/Cargo.toml' has an unsupported or invalid setting 'reserved env key'"
    );
}
