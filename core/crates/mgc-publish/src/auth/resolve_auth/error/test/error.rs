//! Resolver diagnostic text tests / test nội dung chẩn đoán resolver.

use super::super::{auth_not_found, auth_type_basic_missing, auth_type_token_missing};

#[test]
fn missing_auth_message_is_unchanged() {
    assert_eq!(
        auth_not_found("https://registry.example/").to_string(),
        concat!(
            "Auth not found for registry https://registry.example/ — set MGC_NPM_TOKEN or NPM_TOKEN, ",
            ".npmrc, or mgc.toml [registry]"
        )
    );
}

#[test]
fn missing_forced_token_message_is_unchanged() {
    assert_eq!(
        auth_type_token_missing("https://registry.example/").to_string(),
        "registry https://registry.example/ declares auth_type = \"token\" but mgc.toml [registry] has no token"
    );
}

#[test]
fn missing_forced_basic_message_is_unchanged() {
    assert_eq!(
        auth_type_basic_missing("https://registry.example/").to_string(),
        concat!(
            "registry https://registry.example/ declares auth_type = \"basic\" but ",
            "mgc.toml has no [registry] username/password"
        )
    );
}
