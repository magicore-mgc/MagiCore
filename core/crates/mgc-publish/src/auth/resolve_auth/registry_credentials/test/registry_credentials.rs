//! Configured registry auth tests / test auth của registry config.

use mgc_config::registry::Registry;

use super::super::resolve_registry_credentials;

#[test]
fn basic_mode_uses_configured_basic_credentials() {
    let mut registry = Registry::new("registry".into(), "https://registry.example/".into());
    registry.token = Some("ignored-token".into());
    registry.username = Some("user".into());
    registry.password = Some("pass".into());
    registry.auth_type = Some("basic".into());

    let auth = resolve_registry_credentials("https://registry.example/", Some(&registry))
        .unwrap()
        .unwrap();
    assert_eq!(auth.token, None);
    assert_eq!(auth.username.as_deref(), Some("user"));
    assert_eq!(auth.password.as_deref(), Some("pass"));
}

#[test]
fn forced_missing_token_returns_the_existing_diagnostic() {
    let mut registry = Registry::new("registry".into(), "https://registry.example/".into());
    registry.auth_type = Some("token".into());

    let error = resolve_registry_credentials("https://registry.example/", Some(&registry))
        .unwrap_err()
        .to_string();
    assert!(error.contains("auth_type = \"token\""), "{error}");
}
