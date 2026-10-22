//! Tests for registry URL identity rules.
//! Test các quy tắc nhận diện URL registry.

use super::*;

#[test]
fn legacy_identity_ignores_ascii_case_and_trailing_slashes() {
    assert!(same_registry_url(
        "HTTPS://REGISTRY.EXAMPLE/team/",
        "https://registry.example/TEAM",
        true,
    ));
}

#[test]
fn checked_identity_normalizes_authority_and_default_port() {
    assert!(same_registry_url(
        "HTTPS://REGISTRY.EXAMPLE:443/team/",
        "https://registry.example/team",
        false,
    ));
}

#[test]
fn checked_identity_preserves_path_case_query_and_fragment() {
    assert!(!same_registry_url(
        "https://registry.example/Team",
        "https://registry.example/team",
        false,
    ));
    assert!(!same_registry_url(
        "https://registry.example/team?channel=one",
        "https://registry.example/team?channel=two",
        false,
    ));
    assert!(!same_registry_url(
        "https://registry.example/team#one",
        "https://registry.example/team#two",
        false,
    ));
}

#[test]
fn checked_identity_rejects_malformed_urls() {
    assert!(!same_registry_url(
        "not a URL",
        "https://registry.example",
        false
    ));
}
