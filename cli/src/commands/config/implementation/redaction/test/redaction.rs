use super::super::*;

#[test]
fn config_get_redacts_sensitive_keys_from_all_sources() {
    assert_eq!(
        display_config_value("MGC_REGISTRY_TOKEN", "actual-secret"),
        "***"
    );
    assert_eq!(display_config_value("_authToken", "actual-secret"), "***");
    assert_eq!(
        display_config_value("npm_config__auth", "actual-secret"),
        "***"
    );
    assert_eq!(
        display_config_value("authorization", "actual-secret"),
        "***"
    );
    assert_eq!(display_config_value("private_key", "actual-secret"), "***");
    assert_eq!(
        display_config_value("registry", "https://registry.example"),
        "https://registry.example"
    );
}

#[test]
fn toml_config_list_redacts_sensitive_values() {
    let rendered = redact_toml_config(
        "name = \"sample\"\napi_token = \"secret\"\n[registry]\npassword = \"p4ss\"\nurl = \"https://example.test\"\n[[sources]]\nendpoint = \"https://registry.example\"\naccess_token = \"inline-secret\"\n",
    )
    .unwrap();

    assert!(rendered.contains("name = \"sample\""));
    assert!(rendered.contains("api_token = \"***\""));
    assert!(rendered.contains("password = \"***\""));
    assert!(rendered.contains("url = \"https://example.test\""));
    assert!(rendered.contains("access_token = \"***\""));
    assert!(!rendered.contains("secret"));
    assert!(!rendered.contains("p4ss"));
}

#[test]
fn is_sensitive_catches_token_keys() {
    assert!(is_sensitive("_authToken"));
    assert!(is_sensitive("npm_token"));
    assert!(is_sensitive("my_password"));
    assert!(is_sensitive("_auth"));
    assert!(is_sensitive("authorization"));
    assert!(is_sensitive("private_key"));
    assert!(is_sensitive("api_key"));
    assert!(is_sensitive("signing_key"));
    assert!(is_sensitive("private-key"));
    assert!(is_sensitive("api-key"));
    assert!(is_sensitive("registry.private-key"));
    assert!(is_sensitive("apiKey"));
    assert!(is_sensitive("APIKey"));
    assert!(is_sensitive("privateKey"));
    assert!(is_sensitive("accessKey"));
    assert!(!is_sensitive("release_public_key"));
    assert!(!is_sensitive("release-public-key"));
    assert!(!is_sensitive("releasePublicKey"));
    assert!(!is_sensitive("registry"));
    assert!(!is_sensitive("ecosystem"));
}

#[test]
fn is_sensitive_not_false_positive_on_normal_keys() {
    assert!(!is_sensitive("version"));
    assert!(!is_sensitive("name"));
    assert!(!is_sensitive("mode"));
}

#[test]
fn is_sensitive_catches_short_password_spellings() {
    for key in [
        "pass",
        "db_pass",
        "passwd",
        "user_passwd",
        "pwd",
        "deploy_pwd",
        "key",
        "api_key",
        "my_token",
        "secret",
        "auth",
        "registry_auth",
    ] {
        assert!(is_sensitive(key), "{key} must be treated as sensitive");
    }
}

#[test]
fn is_sensitive_leaves_ordinary_words_visible() {
    // `bypass_proxy` contains "pass" as a substring but is not a secret;
    // `monkey`/`keyboard` contain "key" but are ordinary words.
    for key in ["bypass_proxy", "monkey", "keyboard", "registry", "timeout"] {
        assert!(!is_sensitive(key), "{key} must stay visible");
    }
}
